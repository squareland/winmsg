use std::ptr::NonNull;
use bitfield::bitfield;
use winapi::shared::minwindef::{FALSE, LPARAM, TRUE, UINT, WORD, WPARAM};
use winapi::shared::windef::{HDC, HICON, HWND, RECT};
use winapi::um::winuser::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawEvent {
    pub msg: UINT,
    pub w_param: WPARAM,
    pub l_param: LPARAM
}

#[repr(C)]
union UWM {
    raw: RawEvent,
    typed: WindowMessage
}

mod modifiers {
    use winapi::shared::minwindef::*;
    use winapi::um::winuser::*;

    const CONTROL: WPARAM = MK_CONTROL;
    const L_BUTTON: WPARAM = MK_LBUTTON;
    const M_BUTTON: WPARAM = MK_MBUTTON;
    const R_BUTTON: WPARAM = MK_RBUTTON;
    const SHIFT: WPARAM = MK_SHIFT;
    const X_BUTTON1: WPARAM = MK_XBUTTON1;
    const X_BUTTON2: WPARAM = MK_XBUTTON2;
}

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct MousePos {
    pub x: i16,
    pub y: i16
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum MouseActivation {
    Activate = MA_ACTIVATE,
    ActivateAndEat = MA_ACTIVATEANDEAT,
    NoActivate = MA_NOACTIVATE,
    NoActivateAndEat = MA_NOACTIVATEANDEAT,
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum WindowActivation {
    Active = WA_ACTIVE,
    ClickActive = WA_CLICKACTIVE,
    Inactive = WA_INACTIVE,
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct KeyInfo(u64); impl Debug;
    u32;
    pub repeat_count, _: 15, 0;
    pub scan_code, _: 23, 16;
    pub extended, _: 24;
    _reserved, _: 28, 25;
    pub context_code, _: 29;
    pub previous_state, _: 30;
    pub transition_state, _: 31;
}

#[derive(Debug, Copy, Clone)]
pub struct KeyMessage {
    pub up: bool,
    pub sys: bool,
    pub code: WPARAM,
    pub info: KeyInfo
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MouseButton {
    Left, Right, Middle, X(WORD)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MouseButtonAction {
    Down,
    Up,
    DoubleClick
}

#[derive(Debug, Copy, Clone)]
pub struct MouseButtonMessage {
    pub action: MouseButtonAction,
    pub button: MouseButton,
    pub pos: MousePos,
    pub modifiers: WPARAM,
}

#[repr(u64)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum IconSize {
    Small = ICON_SMALL as _,
    Big = ICON_BIG as _,
    Small2 = ICON_SMALL2 as _,
}

#[repr(isize)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum WindowShown {
    OtherUnZoom = SW_OTHERUNZOOM,
    OtherZoom = SW_OTHERZOOM,
    ParentClosing = SW_PARENTCLOSING,
    ParentOpening = SW_PARENTOPENING,
}

#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum WindowResizing {
    MaxHide = SIZE_MAXHIDE,
    Maximized = SIZE_MAXIMIZED,
    MaxShow = SIZE_MAXSHOW,
    Minimized = SIZE_MINIMIZED,
    Restored = SIZE_RESTORED,
}

#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum PowerEvent {
    ApmPowerStatusChange = PBT_APMPOWERSTATUSCHANGE,
    ApmResumeAutomatic = PBT_APMRESUMEAUTOMATIC,
    ApmResumeSuspend = PBT_APMRESUMESUSPEND,
    ApmSuspend = PBT_APMSUSPEND,
    PowerSettingsChange = PBT_POWERSETTINGCHANGE,
}

#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum GwlStyle {
    ExStyle = GWL_EXSTYLE,
    Style = GWL_STYLE,
}

#[cfg(target_pointer_width = "64")]
const _NC_SIZE_ASSERT: [u8; 16] = [0; size_of::<NcSizeParams>()];
#[cfg(target_pointer_width = "32")]
const _NC_SIZE_ASSERT: [u8; 8] = [0; size_of::<NcSizeParams>()];

#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum NcSizeParams {
    ValidClientArea {
        data: Option<NonNull<NCCALCSIZE_PARAMS>>
    } = TRUE as _,
    Rect {
        data: Option<NonNull<RECT>>
    } = FALSE as _,
}


#[cfg(target_pointer_width = "64")]
const _RAW_SIZE_ASSERT: [u8; 24] = [0; size_of::<RawEvent>()];
#[cfg(target_pointer_width = "32")]
const _RAW_SIZE_ASSERT: [u8; 12] = [0; size_of::<RawEvent>()];
const _WM_SIZE_ASSERT: [u8; 24] = [0; size_of::<WindowMessage>()];

#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
    Message(WindowMessage),
    User(RawEvent),
    App(RawEvent),
    String(RawEvent),
    Reserved(RawEvent),
}

impl WindowEvent {
    pub fn parse(msg: UINT, w_param: WPARAM, l_param: LPARAM) -> Self {
        const WM_USER_1: UINT = WM_USER - 1;
        const WM_APP_1: UINT = WM_APP - 1;
        const WM_STRING: UINT = 0xC000;
        const WM_STRING_1: UINT = WM_STRING - 1;
        const WM_RESERVED: UINT = 0xFFFF;
        const WM_RESERVED_1: UINT = WM_RESERVED - 1;
        match msg {
            0..=WM_USER_1 => {
                Self::Message(unsafe {
                    UWM {
                        raw: RawEvent { msg, w_param, l_param }
                    }.typed
                })
            },
            WM_USER..=WM_APP_1 => {
                Self::User(RawEvent { msg: msg - WM_USER, w_param, l_param })
            },
            WM_APP..=WM_STRING_1 => {
                Self::App(RawEvent { msg: msg - WM_APP, w_param, l_param })
            },
            WM_STRING..=WM_RESERVED_1 => {
                Self::String(RawEvent { msg: msg - WM_APP, w_param, l_param })
            },
            WM_RESERVED.. => {
                Self::Reserved(RawEvent { msg: msg - WM_RESERVED, w_param, l_param })
            }
        }
    }
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum WindowMessage {
    Null = WM_NULL,
    Create {
        _unused: WPARAM,
        data: Option<NonNull<CREATESTRUCTA>> //CREATESTRUCTW?
    } = WM_CREATE,
    Destroy = WM_DESTROY,
    Move {
        _unused: WPARAM,
        x: i16,
        y: i16,
        _unused2: u32
    } = WM_MOVE,
    Size {
        resizing: WindowResizing,
        width: i16,
        height: i16,
        _unused: u32
    } = WM_SIZE,
    Activate {
        activated: u16,
        state: WindowActivation,
        window: HWND
    } = WM_ACTIVATE,
    SetFocus {
        window: Option<NonNull<HWND>>
    } = WM_SETFOCUS,
    KillFocus = WM_KILLFOCUS,
    Enable = WM_ENABLE,
    SetRedraw = WM_SETREDRAW,
    SetText = WM_SETTEXT,
    GetText = WM_GETTEXT,
    GetTextLength = WM_GETTEXTLENGTH,
    Paint = WM_PAINT,
    Close = WM_CLOSE,
    QueryEndSession = WM_QUERYENDSESSION,
    QueryOpen = WM_QUERYOPEN,
    EndSession = WM_ENDSESSION,
    Quit = WM_QUIT,
    EraseBackground {
        dc: HDC,
        _unused: LPARAM
    } = WM_ERASEBKGND,
    SysColorChange = WM_SYSCOLORCHANGE,
    ShowWindow {
        shown: WPARAM,
        status: LPARAM
    } = WM_SHOWWINDOW,
    SettingChange = WM_SETTINGCHANGE,
    DevModeChange = WM_DEVMODECHANGE,
    ActivateApp {
        activated: WPARAM,
        thread: LPARAM
    } = WM_ACTIVATEAPP,
    FontChange = WM_FONTCHANGE,
    TimeChange = WM_TIMECHANGE,
    CancelMode = WM_CANCELMODE,
    SetCursor {
        window: HWND,
        hit_test: WORD,
        trigger_message: WORD,
        _unused: u32
    } = WM_SETCURSOR,
    MouseActivate {
        top_window: HWND,
        activation: LPARAM
    } = WM_MOUSEACTIVATE,
    ChildActivate = WM_CHILDACTIVATE,
    QueueSync = WM_QUEUESYNC,
    GetMinMaxInfo {
        _unused: WPARAM,
        data: Option<NonNull<MINMAXINFO>>
    } = WM_GETMINMAXINFO,
    PaintIcon = WM_PAINTICON,
    IconEraseBackground = WM_ICONERASEBKGND,
    NextDialogCtl = WM_NEXTDLGCTL,
    SpoolerStatus = WM_SPOOLERSTATUS,
    DrawItem = WM_DRAWITEM,
    MeasureItem = WM_MEASUREITEM,
    DeleteItem = WM_DELETEITEM,
    VKeyToItem = WM_VKEYTOITEM,
    CharToItem = WM_CHARTOITEM,
    SetFont = WM_SETFONT,
    GetFont = WM_GETFONT,
    SetHotkey = WM_SETHOTKEY,
    GetHotkey = WM_GETHOTKEY,
    QueryDragIcon = WM_QUERYDRAGICON,
    CompareItem = WM_COMPAREITEM,
    GetObject = WM_GETOBJECT,
    Compacting = WM_COMPACTING,
    CommNotify = WM_COMMNOTIFY,
    WindowPosChanging {
        _unused: WPARAM,
        data: Option<NonNull<WINDOWPOS>>
    } = WM_WINDOWPOSCHANGING,
    WindowPosChanged {
        _unused: WPARAM,
        data: Option<NonNull<WINDOWPOS>>
    } = WM_WINDOWPOSCHANGED,
    Power = WM_POWER,
    CopyData = WM_COPYDATA,
    CancelJournal = WM_CANCELJOURNAL,
    Notify = WM_NOTIFY,
    InputLangChangeRequest = WM_INPUTLANGCHANGEREQUEST,
    InputLangChange = WM_INPUTLANGCHANGE,
    TCard = WM_TCARD,
    Help = WM_HELP,
    UserChanged = WM_USERCHANGED,
    NotifyFormat = WM_NOTIFYFORMAT,
    ContextMenu = WM_CONTEXTMENU,
    StyleChanging {
        style: GwlStyle,
        _unused: u32,
        data: Option<NonNull<STYLESTRUCT>>
    } = WM_STYLECHANGING,
    StyleChanged {
        style: GwlStyle,
        _unused: u32,
        data: Option<NonNull<STYLESTRUCT>>
    } = WM_STYLECHANGED,
    DisplayChange = WM_DISPLAYCHANGE,
    GetIcon {
        size: IconSize,
        dpi: LPARAM
    } = WM_GETICON,
    SetIcon {
        size: IconSize,
        icon: HICON
    } = WM_SETICON,
    NcCreate = WM_NCCREATE,
    NcDestroy = WM_NCDESTROY,
    NcCalcSize {
        params: NcSizeParams,
    } = WM_NCCALCSIZE,
    NcHitTest {
        _unused: WPARAM,
        pos: MousePos
    } = WM_NCHITTEST,
    NcPaint {
        update_region: WPARAM,
        _unused: LPARAM
    } = WM_NCPAINT,
    NcActivate {
        update_icon: WPARAM,
        update_region: LPARAM
    } = WM_NCACTIVATE,
    GetDlgCode = WM_GETDLGCODE,
    SyncPaint = WM_SYNCPAINT,
    NcMouseMove {
        hit_test: WPARAM,
        pos: MousePos
    } = WM_NCMOUSEMOVE,
    NclButtonDown = WM_NCLBUTTONDOWN,
    NclButtonUp = WM_NCLBUTTONUP,
    NclButtonDblClk = WM_NCLBUTTONDBLCLK,
    NcRButtonDown = WM_NCRBUTTONDOWN,
    NcRButtonUp = WM_NCRBUTTONUP,
    NcRButtonDblClk = WM_NCRBUTTONDBLCLK,
    NcMButtonDown = WM_NCMBUTTONDOWN,
    NcMButtonUp = WM_NCMBUTTONUP,
    NcMButtonDblClk = WM_NCMBUTTONDBLCLK,
    NcXButtonDown = WM_NCXBUTTONDOWN,
    NcXButtonUp = WM_NCXBUTTONUP,
    NcXButtonDblClk = WM_NCXBUTTONDBLCLK,
    InputDeviceChange = WM_INPUT_DEVICE_CHANGE,
    Input = WM_INPUT,
    KeyDown {
        key_code: WPARAM,
        info: KeyInfo
    } = WM_KEYDOWN,
    KeyUp {
        key_code: WPARAM,
        info: KeyInfo
    } = WM_KEYUP,
    Char = WM_CHAR,
    DeadChar = WM_DEADCHAR,
    SysKeyDown {
        key_code: WPARAM,
        info: KeyInfo
    } = WM_SYSKEYDOWN,
    SysKeyUp {
        key_code: WPARAM,
        info: KeyInfo
    } = WM_SYSKEYUP,
    SysChar = WM_SYSCHAR,
    SysDeadChar = WM_SYSDEADCHAR,
    UniChar = WM_UNICHAR,
    ImeStartComposition = WM_IME_STARTCOMPOSITION,
    ImeEndComposition = WM_IME_ENDCOMPOSITION,
    ImeComposition = WM_IME_COMPOSITION,
    InitDialog = WM_INITDIALOG,
    Command = WM_COMMAND,
    SysCommand = WM_SYSCOMMAND,
    TIMER = WM_TIMER,
    HScroll = WM_HSCROLL,
    VScroll = WM_VSCROLL,
    InitMenu = WM_INITMENU,
    InitMenuPopup = WM_INITMENUPOPUP,
    Gesture = WM_GESTURE,
    GestureNotify = WM_GESTURENOTIFY,
    MenuSelect = WM_MENUSELECT,
    MenuChar = WM_MENUCHAR,
    EnterIdle = WM_ENTERIDLE,
    MenuRButtonUp = WM_MENURBUTTONUP,
    MenuDrag = WM_MENUDRAG,
    MenuGetObject = WM_MENUGETOBJECT,
    UninitMenuPopup = WM_UNINITMENUPOPUP,
    MenuCommand = WM_MENUCOMMAND,
    ChangeUiState = WM_CHANGEUISTATE,
    UpdateUiState = WM_UPDATEUISTATE,
    QueryUiState = WM_QUERYUISTATE,
    CtlColorMsgBox = WM_CTLCOLORMSGBOX,
    CtlColorEdit = WM_CTLCOLOREDIT,
    CtlColorListBox = WM_CTLCOLORLISTBOX,
    CtlColorBtn = WM_CTLCOLORBTN,
    CtlColorDlg = WM_CTLCOLORDLG,
    CtlColorScrollbar = WM_CTLCOLORSCROLLBAR,
    CtlColorStatic = WM_CTLCOLORSTATIC,
    MouseMove {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_MOUSEMOVE,
    LButtonDown {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_LBUTTONDOWN,
    LButtonUp {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_LBUTTONUP,
    LButtonDblClk {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_LBUTTONDBLCLK,
    RButtonDown {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_RBUTTONDOWN,
    RButtonUp {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_RBUTTONUP,
    RButtonDblClk {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_RBUTTONDBLCLK,
    MButtonDown {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_MBUTTONDOWN,
    MButtonUp {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_MBUTTONUP,
    MButtonDblClk {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_MBUTTONDBLCLK,
    MouseWheel {
        modifiers: WORD,
        delta: WORD,
        pos: MousePos
    } = WM_MOUSEWHEEL,
    XButtonDown {
        modifiers: WORD,
        button: WORD,
        pos: MousePos
    } = WM_XBUTTONDOWN,
    XButtonUp {
        modifiers: WORD,
        button: WORD,
        pos: MousePos
    } = WM_XBUTTONUP,
    XButtonDblClk {
        modifiers: WORD,
        button: WORD,
        pos: MousePos
    } = WM_XBUTTONDBLCLK,
    NcUahDrawCaption {
        w: WPARAM,
        l: LPARAM,
    } = 0x00AE /*WM_NCUAHDRAWCAPTION*/,
    NcUahDrawFrame {
        w: WPARAM,
        l: LPARAM,
    } = 0x00AF /*WM_NCUAHDRAWCAPTION */,
    MouseHWheel {
        modifiers: WORD,
        delta: WORD,
        pos: MousePos
    } = WM_MOUSEHWHEEL,
    ParentNotify = WM_PARENTNOTIFY,
    EnterMenuLoop = WM_ENTERMENULOOP,
    ExitMenuLoop = WM_EXITMENULOOP,
    NextMenu = WM_NEXTMENU,
    Sizing = WM_SIZING,
    CaptureChanged {
        _unused: WPARAM,
        window: HWND
    } = WM_CAPTURECHANGED,
    Moving = WM_MOVING,
    PowerBroadcast {
        event: PowerEvent,
        data: Option<NonNull<POWERBROADCAST_SETTING>>
    } = WM_POWERBROADCAST,
    DeviceChange = WM_DEVICECHANGE,
    MdiCreate = WM_MDICREATE,
    MdiDestroy = WM_MDIDESTROY,
    MdiActivate = WM_MDIACTIVATE,
    MdiRestore = WM_MDIRESTORE,
    MdiNext = WM_MDINEXT,
    MdiMaximize = WM_MDIMAXIMIZE,
    MdiTile = WM_MDITILE,
    MdiCascade = WM_MDICASCADE,
    MdiIconArrange = WM_MDIICONARRANGE,
    MdiGetActive = WM_MDIGETACTIVE,
    MdiSetMenu = WM_MDISETMENU,
    EnterSizeMove = WM_ENTERSIZEMOVE,
    ExitSizeMove = WM_EXITSIZEMOVE,
    DropFiles = WM_DROPFILES,
    MdiRefreshMenu = WM_MDIREFRESHMENU,
    PointerDeviceChange = WM_POINTERDEVICECHANGE,
    PointerDeviceInRange = WM_POINTERDEVICEINRANGE,
    PointerDeviceOutOfRange = WM_POINTERDEVICEOUTOFRANGE,
    Touch = WM_TOUCH,
    NcPointerUpdate = WM_NCPOINTERUPDATE,
    NcPointerDown = WM_NCPOINTERDOWN,
    NcPointerUp = WM_NCPOINTERUP,
    PointerUpdate = WM_POINTERUPDATE,
    POINTERDOWN = WM_POINTERDOWN,
    POINTERUP = WM_POINTERUP,
    POINTERENTER = WM_POINTERENTER,
    POINTERLEAVE = WM_POINTERLEAVE,
    PointerActivate = WM_POINTERACTIVATE,
    PointerCaptureChanged = WM_POINTERCAPTURECHANGED,
    TouchHitTesting = WM_TOUCHHITTESTING,
    PointerWheel = WM_POINTERWHEEL,
    PointerHWheel = WM_POINTERHWHEEL,
    PointerRoutedTo = WM_POINTERROUTEDTO,
    PointerRoutedAway = WM_POINTERROUTEDAWAY,
    PointerRoutedReleased = WM_POINTERROUTEDRELEASED,
    ImeSetContext {
        active: WPARAM,
        display_options: LPARAM
    } = WM_IME_SETCONTEXT,
    ImeNotify {
        window: HWND,
        command: LPARAM
    } = WM_IME_NOTIFY,
    ImeControl = WM_IME_CONTROL,
    ImeCompositionFull = WM_IME_COMPOSITIONFULL,
    ImeSelect = WM_IME_SELECT,
    ImeChar = WM_IME_CHAR,
    ImeRequest = WM_IME_REQUEST,
    ImeKeydown = WM_IME_KEYDOWN,
    ImeKeyup = WM_IME_KEYUP,
    MouseHover {
        modifiers: WPARAM,
        pos: MousePos
    } = WM_MOUSEHOVER,
    MouseLeave = WM_MOUSELEAVE,
    NcMouseHover = WM_NCMOUSEHOVER,
    NcMouseLeave = WM_NCMOUSELEAVE,
    WtsSessionChange = WM_WTSSESSION_CHANGE,
    TabletFirst = WM_TABLET_FIRST,
    TabletLast = WM_TABLET_LAST,
    DpiChanged = WM_DPICHANGED,
    DpiChangedBeforeParent = WM_DPICHANGED_BEFOREPARENT,
    DpiChangedAfterParent = WM_DPICHANGED_AFTERPARENT,
    GetDpiScaledSize = WM_GETDPISCALEDSIZE,
    Cut = WM_CUT,
    Copy = WM_COPY,
    Paste = WM_PASTE,
    Clear = WM_CLEAR,
    Undo = WM_UNDO,
    RenderFormat = WM_RENDERFORMAT,
    RenderAllFormats = WM_RENDERALLFORMATS,
    DestroyClipboard = WM_DESTROYCLIPBOARD,
    DrawClipboard = WM_DRAWCLIPBOARD,
    PaintClipboard = WM_PAINTCLIPBOARD,
    VScrollClipboard = WM_VSCROLLCLIPBOARD,
    SizeClipboard = WM_SIZECLIPBOARD,
    AskCbFormatName = WM_ASKCBFORMATNAME,
    ChangeCbChain = WM_CHANGECBCHAIN,
    HScrollClipboard = WM_HSCROLLCLIPBOARD,
    QueryNewPalette = WM_QUERYNEWPALETTE,
    PaletteIsChanging = WM_PALETTEISCHANGING,
    PaletteChanged = WM_PALETTECHANGED,
    Hotkey = WM_HOTKEY,
    Print = WM_PRINT,
    PrintClient = WM_PRINTCLIENT,
    AppCommand = WM_APPCOMMAND,
    ThemeChanged = WM_THEMECHANGED,
    ClipboardUpdate = WM_CLIPBOARDUPDATE,
    DwmCompositionChanged = WM_DWMCOMPOSITIONCHANGED,
    DwmNcRenderingChanged = WM_DWMNCRENDERINGCHANGED,
    DwmColorizationColorChanged = WM_DWMCOLORIZATIONCOLORCHANGED,
    DwmWindowMaximizedChange = WM_DWMWINDOWMAXIMIZEDCHANGE,
    DwmSendIconIcThumbnail = WM_DWMSENDICONICTHUMBNAIL,
    DwmSendIconIcLivePreviewBitmap = WM_DWMSENDICONICLIVEPREVIEWBITMAP,
    GetTitleBarInfoEx = WM_GETTITLEBARINFOEX,
    HandHeldFirst = WM_HANDHELDFIRST,
    HandHeldLast = WM_HANDHELDLAST,
    AfxFirst = WM_AFXFIRST,
    AfxLast = WM_AFXLAST,
    PenWinFirst = WM_PENWINFIRST,
    PenWinLast = WM_PENWINLAST
}

impl WindowMessage {
    pub fn as_key(&self) -> Option<KeyMessage> {
        match *self {
            WindowMessage::SysKeyUp { key_code, info } => Some(KeyMessage { code: key_code, info, sys: true, up: true }),
            WindowMessage::KeyUp { key_code, info } => Some(KeyMessage { code: key_code, info, sys: false, up: true }),
            WindowMessage::SysKeyDown { key_code, info } => Some(KeyMessage { code: key_code, info, sys: true, up: false }),
            WindowMessage::KeyDown { key_code, info } => Some(KeyMessage { code: key_code, info, sys: false, up: false }),
            _ => None
        }
    }

    pub fn as_mouse_button(&self) -> Option<MouseButtonMessage> {
        match *self {
            WindowMessage::LButtonDown { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Down,
                button: MouseButton::Left,
                pos,
                modifiers,
            }),
            WindowMessage::LButtonUp { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Up,
                button: MouseButton::Left,
                pos,
                modifiers,
            }),
            WindowMessage::LButtonDblClk { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::DoubleClick,
                button: MouseButton::Left,
                pos,
                modifiers,
            }),
            WindowMessage::RButtonDown { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Down,
                button: MouseButton::Right,
                pos,
                modifiers,
            }),
            WindowMessage::RButtonUp { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Up,
                button: MouseButton::Right,
                pos,
                modifiers,
            }),
            WindowMessage::RButtonDblClk { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::DoubleClick,
                button: MouseButton::Right,
                pos,
                modifiers,
            }),
            WindowMessage::MButtonDown { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Down,
                button: MouseButton::Middle,
                pos,
                modifiers,
            }),
            WindowMessage::MButtonUp { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::Up,
                button: MouseButton::Middle,
                pos,
                modifiers,
            }),
            WindowMessage::MButtonDblClk { pos, modifiers } => Some(MouseButtonMessage {
                action: MouseButtonAction::DoubleClick,
                button: MouseButton::Middle,
                pos,
                modifiers,
            }),
            WindowMessage::XButtonDown { pos, modifiers, button } => Some(MouseButtonMessage {
                action: MouseButtonAction::Down,
                button: MouseButton::X(button),
                pos,
                modifiers: modifiers as _,
            }),
            WindowMessage::XButtonUp { pos, modifiers, button } => Some(MouseButtonMessage {
                action: MouseButtonAction::Up,
                button: MouseButton::X(button),
                pos,
                modifiers: modifiers as _,
            }),
            WindowMessage::XButtonDblClk { pos, modifiers, button } => Some(MouseButtonMessage {
                action: MouseButtonAction::DoubleClick,
                button: MouseButton::X(button),
                pos,
                modifiers: modifiers as _,
            }),
            _ => None
        }
    }
}