//! Host-mode spy capture: what arming writes, what a refusal or a missing
//! feature leaves behind, the edges the spy mask turns into, and the restore
//! and wake re-arm.

use std::sync::RwLock;
use std::time::Duration;

use hidpp::channel::HidppChannel;
use hidpp::nibble::U4;
use openlogi_core::binding::SpyModel;

use super::*;
use crate::device_io_channel;
use crate::replay::ReplayChannelHandle;
use crate::session::capture::Rearm;

/// Feature indices the scripted mouse answers for `0x8110`, `0x8100` and
/// `0x1d4b`.
const FILTER: u8 = 4;
const PROFILES: u8 = 5;
const WIRELESS: u8 = 6;
/// `0x8100` mode bytes.
const ONBOARD: u8 = 1;
const HOST: u8 = 2;
/// The mapping the G502 X Plus reported in its live dump.
const NATIVE: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

/// The scripted mouse's firmware state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Firmware {
    mode: u8,
    mapping: [u8; 11],
    spying: bool,
}

/// How the scripted mouse answers.
#[derive(Clone, Copy, Default)]
struct Behavior {
    /// Report no `0x8100`.
    without_profiles: bool,
    /// Answer `setMode` with `InvalidArgument`.
    refuse_host_mode: bool,
    /// Answer `stopSpy` with `InvalidArgument` while the spy is not running.
    refuse_idle_stop: bool,
}

/// Whether the channel to the scripted mouse delivers every request. Fixed
/// when the channel is built.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Transport {
    Healthy,
    /// Fail every `setMapping` before the mouse sees it.
    FailMappingWrites,
}

fn invalid_argument(request: &[u8], response: &mut [u8]) {
    response[2] = 0xff;
    response[3] = request[2];
    response[4] = request[3];
    response[5] = 0x02;
}

fn is_mapping_write(report: &[u8]) -> bool {
    report[2] == FILTER && report[3] >> 4 == 4
}

/// The scripted mouse: its firmware state and how it answers, which a test
/// may change between requests.
#[derive(Clone, Copy)]
struct Mouse {
    firmware: Firmware,
    behavior: Behavior,
}

fn g502(
    firmware: Firmware,
    behavior: Behavior,
    transport: Transport,
) -> (
    ScriptedRawHidChannel,
    ReplayChannelHandle,
    Arc<Mutex<Mouse>>,
) {
    let state = Arc::new(Mutex::new(Mouse { firmware, behavior }));
    let live = Arc::clone(&state);
    let responder = move |request: &[u8]| {
        let mut response = vec![0; 20];
        response[..4].copy_from_slice(&request[..4]);
        response[0] = 0x11;
        let mut mouse = live.lock().unwrap();
        let behavior = mouse.behavior;
        let firmware = &mut mouse.firmware;
        match (request[2], request[3] >> 4) {
            (0, 1) => response[4] = 4,
            (0, 0) => match u16::from_be_bytes([request[4], request[5]]) {
                0x8110 => response[4] = FILTER,
                0x8100 if !behavior.without_profiles => response[4] = PROFILES,
                0x1d4b => response[4] = WIRELESS,
                _ => {}
            },
            (FILTER, 0) => response[4] = 11,
            (FILTER, 1) => firmware.spying = true,
            (FILTER, 2) if behavior.refuse_idle_stop && !firmware.spying => {
                invalid_argument(request, &mut response);
            }
            (FILTER, 2) => firmware.spying = false,
            (FILTER, 3) => response[4..15].copy_from_slice(&firmware.mapping),
            (FILTER, 4) => firmware.mapping.copy_from_slice(&request[4..15]),
            (PROFILES, 1) if behavior.refuse_host_mode && request[4] == HOST => {
                invalid_argument(request, &mut response);
            }
            (PROFILES, 1) => firmware.mode = request[4],
            (PROFILES, 2) => response[4] = firmware.mode,
            _ => panic!("unexpected capture request: {request:02x?}"),
        }
        Some(response)
    };
    let (raw, handle) = if transport == Transport::FailMappingWrites {
        ScriptedRawHidChannel::with_failing_writes(responder, is_mapping_write)
    } else {
        ScriptedRawHidChannel::with_dynamic_responder(responder)
    };
    (raw, handle, state)
}

fn model() -> &'static SpyModel {
    SpyModel::for_model_key("04099").expect("the G502 X Plus has a spy row")
}

const EXTRAS: [ButtonId; 4] = [
    ButtonId::DpiShift,
    ButtonId::ProfileCycle,
    ButtonId::DpiUp,
    ButtonId::DpiDown,
];

fn spec() -> CaptureSpec {
    CaptureSpec {
        spy: Some(SpyCapture {
            model: model(),
            buttons: EXTRAS.to_vec(),
        }),
        ..CaptureSpec::default()
    }
}

/// The mapping with the G6–G9 slots (bits 4, 8, 9 and 10) zeroed.
const SUPPRESSED: [u8; 11] = [1, 2, 3, 4, 0, 6, 7, 8, 0, 0, 0];

const ON_BOARD: Firmware = Firmware {
    mode: ONBOARD,
    mapping: NATIVE,
    spying: false,
};

struct Armed {
    result: Result<(), CaptureError>,
    controls: ArmedControls,
    channel: Arc<HidppChannel>,
    mouse: Arc<Mutex<Mouse>>,
}

async fn arm(firmware: Firmware, behavior: Behavior, spec: &CaptureSpec) -> Armed {
    arm_over(firmware, behavior, Transport::Healthy, spec).await
}

async fn arm_over(
    firmware: Firmware,
    behavior: Behavior,
    transport: Transport,
    spec: &CaptureSpec,
) -> Armed {
    let (raw, _handle, mouse) = g502(firmware, behavior, transport);
    let channel = scripted_channel(raw).await;
    let device = Device::new(channel.clone(), 0xff).await.unwrap();
    let mut armed = ArmedControls::default();
    let result = arm_controls_into(&device, &channel, 0xff, spec, &mut armed).await;
    Armed {
        result,
        controls: armed,
        channel,
        mouse,
    }
}

fn now(mouse: &Arc<Mutex<Mouse>>) -> Firmware {
    mouse.lock().unwrap().firmware
}

/// The mouse power-cycled, or another tool rewrote it, to `firmware`.
fn set(mouse: &Arc<Mutex<Mouse>>, firmware: Firmware) {
    mouse.lock().unwrap().firmware = firmware;
}

#[tokio::test]
async fn arming_enters_host_mode_suppresses_the_armed_slots_and_starts_the_spy() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.unwrap();
    assert!(armed.controls.spy.is_some());
    assert_eq!(
        now(&armed.mouse),
        Firmware {
            mode: HOST,
            mapping: SUPPRESSED,
            spying: true,
        }
    );
}

#[tokio::test]
async fn without_a_requested_spy_the_mouse_is_not_touched() {
    let armed = arm(ON_BOARD, Behavior::default(), &CaptureSpec::default()).await;
    armed.result.unwrap();
    assert!(armed.controls.spy.is_none());
    assert_eq!(now(&armed.mouse), ON_BOARD);
}

/// Without `0x8100` the mapping would not apply: the buttons stay native and
/// the rest of the session arms.
#[tokio::test]
async fn a_mouse_without_onboard_profiles_stays_native() {
    let behavior = Behavior {
        without_profiles: true,
        ..Behavior::default()
    };
    let armed = arm(ON_BOARD, behavior, &spec()).await;
    armed.result.unwrap();
    assert!(armed.controls.spy.is_none());
    assert_eq!(now(&armed.mouse), ON_BOARD);
}

/// A refused Host mode hands back whatever arming wrote, and the session arms
/// without the spy rather than failing every other capture.
#[tokio::test]
async fn a_refused_host_mode_leaves_the_buttons_native_and_unowned() {
    let behavior = Behavior {
        refuse_host_mode: true,
        refuse_idle_stop: true,
        ..Behavior::default()
    };
    let armed = arm(ON_BOARD, behavior, &spec()).await;
    armed.result.unwrap();
    assert!(armed.controls.spy.is_none());
    assert_eq!(now(&armed.mouse), ON_BOARD);
}

/// A transport failure cannot prove what the mouse applied: arming fails with
/// the spy still owned, for the rollback to hand back.
#[tokio::test]
async fn a_transport_failure_fails_arming_with_the_spy_still_owned() {
    let armed = arm_over(
        ON_BOARD,
        Behavior::default(),
        Transport::FailMappingWrites,
        &spec(),
    )
    .await;
    assert!(armed.result.is_err());
    assert!(armed.controls.spy.is_some());
}

async fn restore_outcome(armed: Armed) -> CaptureSessionOutcome {
    let route = DeviceRoute::Direct {
        vendor_id: 0x046d,
        product_id: 0xc095,
    };
    let registry = ChannelRegistry::default();
    registry.replace_node(
        NodeId::from("mouse-node".to_owned()),
        [route.clone()],
        armed.channel.clone(),
    );
    let shared = SharedChannel::new(armed.channel, route);
    let pending = armed
        .controls
        .into_pending(&shared)
        .expect("a Host-mode capture must be owed a restore");
    pending.allow_current_channel().retry(&registry).await
}

#[tokio::test]
async fn restore_stops_the_spy_and_hands_back_mapping_and_onboard_mode() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.as_ref().unwrap();
    let mouse = Arc::clone(&armed.mouse);
    assert!(matches!(
        restore_outcome(armed).await,
        CaptureSessionOutcome::Restored
    ));
    assert_eq!(now(&mouse), ON_BOARD);
}

/// Host mode found on arming is a session that never tore down. Handing it
/// back would leave the buttons it suppressed dead, so restore returns the
/// mouse to its onboard profiles.
#[tokio::test]
async fn restore_returns_a_leftover_host_mode_to_onboard() {
    let leftover = Firmware {
        mode: HOST,
        mapping: SUPPRESSED,
        spying: false,
    };
    let armed = arm(leftover, Behavior::default(), &spec()).await;
    armed.result.as_ref().unwrap();
    let mouse = Arc::clone(&armed.mouse);
    assert!(matches!(
        restore_outcome(armed).await,
        CaptureSessionOutcome::Restored
    ));
    assert_eq!(now(&mouse).mode, ONBOARD);
    assert!(!now(&mouse).spying);
}

/// A rollback can owe the restore before the spy ever started; a mouse that
/// refuses to stop a spy that is not running is still handed back.
#[tokio::test]
async fn a_refused_stop_of_an_idle_spy_still_restores() {
    let behavior = Behavior {
        refuse_idle_stop: true,
        ..Behavior::default()
    };
    let armed = arm(ON_BOARD, behavior, &spec()).await;
    armed.result.as_ref().unwrap();
    // The mouse power-cycled: back on its own profiles, spy stopped.
    set(
        &armed.mouse,
        Firmware {
            mode: HOST,
            mapping: SUPPRESSED,
            spying: false,
        },
    );
    let mouse = Arc::clone(&armed.mouse);
    assert!(matches!(
        restore_outcome(armed).await,
        CaptureSessionOutcome::Restored
    ));
    assert_eq!(now(&mouse), ON_BOARD);
}

/// A power cycle drops Host mode, the mapping and the spy; the wake re-arm
/// writes them all again.
#[tokio::test]
async fn rearm_after_wake_takes_host_mode_again() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.as_ref().unwrap();
    set(&armed.mouse, ON_BOARD);
    assert_eq!(armed.controls.rearm().await, Rearm::Done);
    assert_eq!(
        now(&armed.mouse),
        Firmware {
            mode: HOST,
            mapping: SUPPRESSED,
            spying: true,
        }
    );
}

/// A re-arm that cannot finish may leave the buttons suppressed with no spy
/// reporting them: the session must restart rather than wait for a broadcast.
#[tokio::test]
async fn a_failed_rearm_asks_for_a_restart() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.as_ref().unwrap();
    {
        let mut mouse = armed.mouse.lock().unwrap();
        mouse.firmware = ON_BOARD;
        mouse.behavior.refuse_host_mode = true;
    }
    assert_eq!(armed.controls.rearm().await, Rearm::Restart);
}

fn spy_event(device_index: u8, feature_index: u8, software_id: u8, mask: u16) -> v20::Message {
    let mut payload = [0; 16];
    payload[..2].copy_from_slice(&mask.to_be_bytes());
    v20::Message::Long(
        v20::MessageHeader {
            device_index,
            feature_index,
            function_id: U4::from_lo(0),
            software_id: U4::from_lo(software_id),
        },
        payload,
    )
}

/// Through the session's handler: each armed bit that changes is a press or
/// a release. A report from another device, another feature or a request
/// reply is not a spy event.
#[tokio::test]
async fn spy_masks_reach_the_dispatcher_as_button_edges() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.unwrap();
    let capture = GestureCapture::new(armed.controls);
    let (sink, mut inputs) = mpsc::unbounded_channel();
    let handler = capture.report_handler(0xff, sink);
    handler(&spy_event(0xff, FILTER, 0, 0x0200));
    handler(&spy_event(0x01, FILTER, 0, 0x0000));
    handler(&spy_event(0xff, PROFILES, 0, 0x0000));
    handler(&spy_event(0xff, FILTER, 1, 0x0000));
    // G4 (bit 3) is not armed in this session.
    handler(&spy_event(0xff, FILTER, 0, 0x0208));
    handler(&spy_event(0xff, FILTER, 0, 0x0010));
    assert_eq!(
        std::iter::from_fn(|| inputs.try_recv().ok()).collect::<Vec<_>>(),
        [
            CapturedInput::ButtonDown(ButtonId::DpiUp),
            CapturedInput::ButtonDown(ButtonId::DpiShift),
            CapturedInput::ButtonUp(ButtonId::DpiUp),
        ]
    );
}

/// A power cycle forgets which buttons were held: after the reset, the first
/// mask is compared against nothing held.
#[tokio::test]
async fn a_wake_reset_forgets_the_last_mask() {
    let armed = arm(ON_BOARD, Behavior::default(), &spec()).await;
    armed.result.unwrap();
    let capture = GestureCapture::new(armed.controls);
    let (sink, mut inputs) = mpsc::unbounded_channel();
    let handler = capture.report_handler(0xff, sink);
    handler(&spy_event(0xff, FILTER, 0, 0x0200));
    capture.reset_input_state();
    handler(&spy_event(0xff, FILTER, 0, 0x0200));
    assert_eq!(
        std::iter::from_fn(|| inputs.try_recv().ok()).collect::<Vec<_>>(),
        [
            CapturedInput::ButtonDown(ButtonId::DpiUp),
            CapturedInput::ButtonDown(ButtonId::DpiUp),
        ]
    );
}

/// End to end through the session: a reconnection whose Host-mode re-arm
/// fails ends the session by itself, and its teardown hands the mouse back to
/// its onboard profiles, so the supervisor can arm a fresh one.
#[tokio::test]
async fn a_failed_wake_rearm_restarts_the_session_with_the_mouse_handed_back() {
    let (raw, handle, mouse) = g502(ON_BOARD, Behavior::default(), Transport::Healthy);
    let incoming = raw.incoming_sender();
    let route = DeviceRoute::Direct {
        vendor_id: 0x046d,
        product_id: 0xc095,
    };
    let registry = ChannelRegistry::default();
    registry.replace_node(
        NodeId::from("mouse-node".to_owned()),
        [route.clone()],
        scripted_channel(raw).await,
    );
    let (_shutdown, shutdown_rx) = oneshot::channel();
    let task = tokio::spawn(async move {
        let (sink, _inputs) = mpsc::unbounded_channel();
        let (_signal, device_io) = device_io_channel();
        let host = CaptureHost {
            sink,
            shutdown: shutdown_rx,
            channel_slot: Arc::new(RwLock::new(None)),
            registry: &registry,
            device_io,
        };
        run_capture_session(route, spec(), host).await
    });

    // Arming has finished once the session looks up `0x1d4b`.
    tokio::time::timeout(Duration::from_secs(2), async {
        while !handle
            .written_reports()
            .iter()
            .any(|report| report[2] == 0 && report[3] >> 4 == 0 && report[4..6] == [0x1d, 0x4b])
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("the session arms");
    assert!(now(&mouse).spying);

    // The mouse power-cycled and now refuses Host mode.
    {
        let mut mouse = mouse.lock().unwrap();
        mouse.firmware = ON_BOARD;
        mouse.behavior.refuse_host_mode = true;
    }
    let mut broadcast = vec![0; 20];
    broadcast[..4].copy_from_slice(&[0x11, 0xff, WIRELESS, 0x00]);
    broadcast[4] = 1;
    incoming.send(broadcast).unwrap();

    let outcome = tokio::time::timeout(Duration::from_secs(2), task)
        .await
        .expect("the session stops by itself")
        .expect("the session task does not panic");
    assert!(matches!(outcome, Ok(CaptureSessionOutcome::Restored)));
    assert_eq!(now(&mouse), ON_BOARD);
}
