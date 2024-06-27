//! ----------------------------------------------------------------------------
//!
//! ## R Desk
//!     R Image, R Editor, R DOS
//!
//! # R Image
//!
//! # R Editor
//!
//!
//! # R Dos
//!     Rustbasic DOS is designed as a command-based graphic emulator for Rustbasic.
//!     Rustbasic enables easy text-based graphic Rust programming.
//!
//!
//! Thanks to Sophia Kang for working together.
//! Thanks to `emilk & Team` for the reference and use of egui. ( https://github.com/emilk/egui )
//! Thanks to `all RUST Team`.
//! Thanks to `all the creators` of the libraries used in this source code.
//! ----------------------------------------------------------------------------

// #![allow(unused_imports)]
use image::EncodableLayout;
use serde::{Deserialize, Serialize}; //  __private::de}; // for as_bytes()
                                     // use zerocopy::AsBytes;

pub mod rchat;
pub mod rchatgemini;
pub mod rchatopenai;
pub mod rcrypto;
pub mod rdos;
pub mod reditor;
pub mod rfilesystem;
pub mod rimage;
pub mod rmaster;

/// TODO :
///
/// Rustbasic DOS is designed a DOS command-based graphic emulator for Rustbasic, a library that enables easy text-based graphic RUST programming.
/// Rustbasic is created using the Rust language and includes various examples and help, including the emulator.
/// With Rustbasic, it is relatively easy to write simple graphic programs using the Rust language.
/// Rustbasic DOS는 Rustbasic을 위한 도스 명령어 기반의 그래픽 에뮬레이터이며, Rustbasic은 쉽게 텍스트기반 그래픽 RUST 프로그래밍을 가능하도록 하는 라이브러리입니다.
/// Rustbasic은 Rust 언어를 사용하여 만들어졌으며, 에뮬레이터를 포함하여 다양한 예제와 도움말이 제공됩니다.
/// Rustbasic을 사용하면 비교적 쉽게 Rust 언어를 사용하여 간단한 그래픽 프로그램을 작성할 수 있습니다.
pub mod rmenu;
pub mod rsetup;
pub mod rspeech;

pub use eframe::egui::*;
// use egui::*;

pub use rchat::*;
pub use rchatgemini::*;
pub use rchatopenai::*;
pub use rcrypto::*;
pub use rdos::*;
pub use reditor::*;
pub use rfilesystem::*;
pub use rimage::*;
pub use rmaster::*;
pub use rmenu::*;
pub use rsetup::*;
pub use rspeech::*;

pub const R_DESK_NAME: &'static str = "R-DESK";
pub const R_DESK_DIRECTORY_NAME: &'static str = r"\RDESK";
pub const R_DESK_VERSION: &'static str = "v0.9.0";
pub const R_DESK_CRYPTO_VERSION: &'static str = "v0.9.0";
pub const R_DESK_CRYPTO_DEFAULT_VERSION: &'static str = "v0.9.0";
pub const R_DESK_CRYPTO_INIT_VERSION: &'static str = "v0.0.0";
pub const R_DESK_USER_DEFAULT: &'static str = "rdesk-aaaaaaaaa";
static RDESK_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub const LIMIT_LOGIN_SECONDS: usize = 3;
pub const LIMIT_CREATE_ID_SECONDS: usize = 6;

#[derive(Default)]
pub struct PromptCommand {
    command: String,
    change_command: bool,
    end_of_cursor: bool,
    onetime_focus: bool,
    twotime_focus: bool,
    is_prompt_before: bool,
    is_prompt_on: bool,
    change_prompt: bool,
    check_processing: bool,
    command_history: Vec<String>,
    command_total: usize,
    command_pos: usize,
}

#[derive(Clone, Default, Debug)]
pub struct MessageWindow {
    exist_time_second: usize,
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
    collapsible_button: bool,
    close_button: bool,
    frame_bg_color: TokenType,
    text_bg_color: TokenType,
    title_color: TokenType,
    title: String,
    message: String,
    message_arc: std::sync::Arc<std::sync::Mutex<String>>,
    leftbutton: String,
    centerbutton: String,
    rightbutton: String,
    result_button: String,
}

#[derive(Clone, Default)]
pub struct FilePannel {
    font_id: FontId,
    md_tooltip_width: f32,
    md_tooltip_height: f32,
    md_tooltip_font_size: f32,
    md_tooltip_font_id: FontId,
}

#[derive(Copy, Clone, Default)]
pub enum Method {
    #[default]
    Get,
    Post,
}

#[derive(Default)]
pub enum Download {
    #[default]
    None,
    InProgress,
    StreamingInProgress {
        response: ehttp::PartialResponse,
        body: Vec<u8>,
    },
    Done(ehttp::Result<ehttp::Response>),
}

// #[allow(dead_code)]
pub struct Crypto {
    input_click_num: usize,
    savetime_rlogin_menu: std::time::Instant,
    menu_lifetime_seconds: usize,
    savetime_login: std::time::Instant,
    login_lifetime_seconds: usize,
    key: chacha20poly1305::Key, // len() : 32 bytes
    nonce: [u8; 12],            // len() : 12 bytes ( 96 bits )
    cipher: chacha20poly1305::ChaCha20Poly1305,
    //    ciphertext: Vec<u8>,
    //    associate: Vec<u8>,
    id_hidden: bool,
    id_onetime_focus: bool,
    crypto_id: Vec<u8>,
    crypto_password: Vec<u8>,
    bishop_idpassword: Option<String>,
    inputvec: Vec<u8>,
    outputvec: Vec<u8>,
    //    test_key: String,
    //    test_nonce: String,
    test_input: String,
}

// #[allow(dead_code)]
#[derive(Default)]
pub struct Ehttp {
    start_trigger: bool, // ehhtp
    method: Method,      // ehttp method
    streaming: bool,     // ehttp streaming
    inprogress: bool,    // ehttp inprogress
    pub download_len: usize,
    download: std::sync::Arc<std::sync::Mutex<Download>>,
    request_url: String,
    request_text: String,
    multipart_boundary: String,
    multipart_bytes: Vec<u8>,
    response_bytes: Vec<u8>,
    response_url: String,
    response_text: String,
    response_content_type: String,
    //    num_bytes: usize,
}

// #[allow(dead_code)]
#[derive(Default)]
pub struct Chat {
    font_id_roompannel: FontId,
    is_joined_chatroom: bool,
    joined_chatroom: ChatTree,
    selected_chatroom: ChatTree,
    visible_chat_login_setup: bool,
    chatroom_user_name: String,
    chatroom_user_sex: String,
    chatroom_user_age: String,
    chatroom_assist_name: String,
    chatroom_assist_sex: String,
    chatroom_assist_age: String,
    language: String,
    system_message: String,
}

#[derive(Default)]
pub struct OpenaiChat {
    ehttp: Ehttp,
    num_received: usize,
    view_response_all: bool,
    auto_hidden_clear: bool,
    is_ongoing_talk: bool,
    is_not_talk: bool,
    // is_joined_chatroom: bool,
    // joined_chatroom: ChatTree,
    // selected_chatroom: ChatTree,
    system_message: String,
    openai_apikey_user: String,
    openai_model: String,
    //    openai_path: String,
    completion_request: CompletionRequest,
    //    completion_response: CompletionResponse,
    error_response: rchat::OpenaiErrorResponse,
    finish_reason: String,
    hidden_text: String,
    input_text: String,
    output_text: String,
    //    o_response: Result<rchat::OpenResponse, String>,
    image_request: ImageRequest,
    revised_prompt: Option<String>,
    b64_json: Option<String>,
    show_image_onetime: bool,
    history: ChatHistory,
}

#[derive(Default)]
pub struct GeminiChat {
    ehttp: Ehttp,
    num_received: usize,
    view_response_all: bool,
    auto_hidden_clear: bool,
    is_ongoing_talk: bool,
    is_not_talk: bool,
    // is_joined_chatroom: bool,
    // joined_chatroom: ChatTree,
    // selected_chatroom: ChatTree,
    system_message: String,
    finish_reason: String,
    hidden_text: String,
    input_text: String,
    output_text: String,
    //    o_response: Result<rchat::OpenResponse, String>,
    vision_request_path: String,
    gemini_apikey_user: String,
    gemini_request: generateContentGemini,
    gemini_response: generateContentResponseGemini,
    gemini_error_response: ErrorResponseGemini,
    history: ChatHistory,
}

#[derive(Default)]
pub struct AiServer {
    ehttp: Ehttp,
    model_text: String,
    // model_vec: Vec<ModelOpenai>,
    model_openai_vec: Vec<ModelOpenai>,
    model_gemini_vec: Vec<ModelGemini>,
    url_path_vec: Vec<String>,
    openai_organization: String,
    apikey_hidden: bool,
    openai_apikey: String,
    gemini_apikey: String,
}

// #[allow(dead_code)]
pub struct AD {
    ehttp: Ehttp,
    started_time: f64,
    reload_count: usize,
    duration_count: usize,
    frame_count: usize,
    url: String,
    uri_before: String,
    click_url: String,
    //    click_url_before: String,
    set_file: String,
    setup: LangStore,
    max_num: usize,
    display_num: usize,
    display_frame_num: usize,
    ad_image_url: Vec<Vec<String>>,
    ad_image_click_url: Vec<Vec<String>>,
    onetime_setup_read: bool,
    onetime_read: bool,
    width: f32,
    height: f32,
    image_data: Option<egui::Image<'static>>,
    is_ad_request_change: bool,
}

#[derive(Default)]
pub struct RImage {
    ehttp: Ehttp,
    image_address: String,
    image_uri_before: String,
    image_path: String,
    image_list: Vec<String>,
    image_filename: String,
    image_filedata: Vec<u8>,
    image_data: Option<egui::Image<'static>>,
    image_scale_default: f32,
    image_scale: f32,
    image_onetime_internet: bool,
    image_onetime_random: bool,
    image_onetime_file: bool,
}

#[derive(Default)]
pub struct Audio {
    audio_bytes: Vec<u8>,
    audio_text: String,
    volume: f32,
    speed: f32,
    seted_volume: Option<f32>,
    seted_speed: Option<f32>,
    manager: Option<kira::manager::AudioManager>,
    handle: Option<kira::sound::static_sound::StaticSoundHandle>,
    state: Option<kira::sound::PlaybackState>,
    data: Option<kira::sound::static_sound::StaticSoundData>,
    //    sink: Option<rodio::Sink>,
    sample_rate: usize,
}

#[derive(Default)]
pub struct Sound {
    ehttp: Ehttp,
    audio: Audio,
    recorder: AudioRecorder,
    sound_file_path: String,
    input_text: String,
    used_no_clear: bool,
    speech_text: String,
    speech_text_pos: usize,
    speech_language: Option<String>,
    speech_using_file: bool,
    speech_dat_path: String,
    google_client: String,
    char_map: std::collections::HashMap<char, Vec<u8>>,
    speech_request: CreateSpeechRequest,
    audio_request: CreateTranscriptionRequest,
}

#[derive(Clone, Default)]
pub struct InputPath {
    path: String,
    is_dir: bool,
    input_path: String,
    onetime_focus: bool,
}

#[derive(Clone, Default)]
pub struct SelectedPath {
    path: String,
    is_dir: bool,
    is_move: bool,
}

#[derive(Clone, Default)]
pub struct FindStringValue {
    find_num: usize,
    global_index: usize,
    line_num: usize,
    column_num: usize,
    line: String,
}
/*
#[derive(Clone, Default)]
pub struct FindString {
    find_input: String,
    find_onetime_focus: bool,
    replace_input: String,
    replace_onetime_focus: bool,
    find_vec: Vec<FindStringValue>,
    find_select_num: usize,
}
*/

#[derive(Default)]
pub struct TextEditData {
    text_changed: bool,
    text_len: usize,
    text: String,
    filename: String,
    text_byte_ansi: bool,
    text_byte: Vec<u8>,
    onetime_focus: bool,
    onetime_move_cursor: bool,
    move_cursor: Option<usize>,
    font_size: f32,
    desired_rows: usize,
    // temporary use
    // row_height: f32,
    row_height_vec: Vec<f32>,
    ccursor: epaint::text::cursor::CCursor,
    ccursor_prev: epaint::text::cursor::CCursor,
    char_line_indexes: Vec<usize>,
    total_lines: usize,
    current_line: usize,
    total_columns: usize,
    current_column: usize,
    current_start_line: usize,
    current_end_line: usize,
    freedom_request_ok: bool,
    freedom_row: usize,
    freedom_column: usize,
    //    find: FindString,
    find_input: String,
    find_onetime_focus: bool,
    replace_input: String,
    replace_onetime_focus: bool,
    find_vec: Vec<FindStringValue>,
    find_select_num: usize,
}

#[derive(Clone, Default, Debug)]
pub struct ScreenData {
    always_on_top: bool,
    decorated: bool,
    decorated_skip_onetime: bool,
    minimized: bool,
    maximized: bool,
    fullscreen: bool,
    //    window_size_onetime_set: bool,
    window_size: Vec2,
    window_size_min: Vec2,
    dark_mode: bool,
    basic_mode: usize,
    sub_mode: usize,
    clearglass_value: u8,
    sunglass_value: u8,
    blackglass_value: u8,
    whiteglass_value: u8,
    button_padding_x: f32,
    button_padding_y: f32,
    item_spacing_x: f32,
    item_spacing_y: f32,
    preload_fontfilename: String,
    preload_fontfiledata: &'static [u8],
    font_adjust: f32,
    font_size_large: f32,
    font_size_normal: f32,
    font_size_small: f32,
    font_size_desk: f32,
    font_size_roompannel: f32,
    font_size_filepannel: f32,
    font_size_microview: f32,
    font_filename: String,
    font_scale: f32,
    font_y_offset_factor: f32,
    font_y_offset: f32,
    font_list: Vec<String>,
    themecolor_dark: Vec<egui::Color32>,
    themecolor_light: Vec<egui::Color32>,
    themecolor_dark_vec: Vec<Vec<egui::Color32>>,
    themecolor_light_vec: Vec<Vec<egui::Color32>>,
    themecolor_language: String,
    leftpanel_width: f32,
    leftpanel_width_min: f32,
    leftpanel_width_default: f32,
    rightpanel_width: f32,
    rightpanel_width_min: f32,
    rightpanel_width_default: f32,
    lang_store: LangStore,
    message_window: MessageWindow,
}

// #[allow(dead_code)]
pub struct Content {
    screen: ScreenData,
    rdesk_exe_path: String,
    rdesk_start_path: String,
    rdesk_path: String,
    rdesk_crypto_path: String,
    rdesk_crypto_user_path: String,
    rdesk_crypto_user_temp_path: String,
    rdesk: PromptCommand,
    rdesk_ccursor: epaint::text::cursor::CCursor,
    check_rdesk_exist: usize,
    savetime_started_limit: std::time::Instant,
    limit_login_seconds: usize,
    limit_create_id_seconds: usize,
    savetime_filetree: std::time::Instant,
    savetime_freecursor: std::time::Instant,
    savetime_dos: std::time::Instant,
    savetime_messagewindow: std::time::Instant,
    rdesk_id: String,
    rdesk_password: String,
    is_system_master: bool,
    crypto: Crypto,
    bishop: Bishop,
    rc4: RC4Plus,
    text_speech: Sound,
    audio_text: Sound,
    google_speech: Sound,
    sound: Sound,
    mictest: Sound,
    frame_history: FrameHistory,
    //    lang_store: LangStore,
    console_name: String,
    console: ConsoleScreen,
    dos: TextEditData,
    rdos: PromptCommand,
    cmd_command: std::process::Command,
    // cmd_child_vec: Vec<Option<std::process::Child>>,
    cmd_child_vec: Vec<CmdChild>,
    cmd_command_vec: Vec<String>,
    chat_text: TextEditData,
    chat_prompt: String,
    chat_is_multiline: bool,
    rchat: PromptCommand,
    chat_tree: Vec<ChatTree>,
    chat: Chat,
    o_chat: OpenaiChat,
    g_chat: GeminiChat,
    o_image: OpenaiChat,
    openai: AiServer,
    gemini: AiServer,
    editor: TextEditData,
    file_tree: Vec<FileTree>,
    file_tree_immediately: bool,
    filepannel: FilePannel,
    selected_path: SelectedPath,
    path_right_click: SelectedPath,
    path_paste: SelectedPath,
    path_copymove: SelectedPath,
    encrypt_copymove: SelectedPath,
    decrypt_copymove: SelectedPath,
    input_newfolder: InputPath,
    input_rename: InputPath,
    reditor_fileopen: InputPath,
    reditor_filesaveas: InputPath,
    rimage_filesaveas: InputPath,
    fontbook: FontBook,
    rimage: RImage,
    ad: AD,
    painting: rimage::Painting,
    browser_path: String,
    status_doswin_hide: bool,
    viewport_master_etc: std::sync::Arc<std::sync::atomic::AtomicBool>,
    viewport_master_etc_immediate: bool,
    viewport_dev_speech_immediate: bool,
    viewport_dev_speech_google_immediate: bool,
    close_requested: bool,
    close_cancelable: bool,
}

#[rustfmt::skip]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum BasicMode {                                            // using bit = 1,2,4,8
                BasicModeNone           = 0x0000000000000000,
    #[default]  RDesk                   = 0x0000000000000001,
                RDeskInternetImage      = 0x0000000000000002,
                RDeskFileImage          = 0x0000000000000004,
                RDos                    = 0x0000000000000010,
                RDosBasic               = 0x0000000000000020,
                RDosNormal              = 0x0000000000000040,
                REditor                 = 0x0000000000000100,
                REditorBasic            = 0x0000000000000200,
                REditorNormal           = 0x0000000000000400,
                REditorEasymark         = 0x0000000000000800,
                RChat                   = 0x0000000000001000,
                RLogin                  = 0x0000000000002000,
                RCryptoDisk             = 0x0000000000004000,
                RMaster                 = 0x0000000000008000,
                SelectLanguage          = 0x0000000000010000,
                FontBook                = 0x0000000000020000,
                ThemeColor              = 0x0000000000040000,
                FramePerSecond          = 0x0000000000080000,
                ClearGlassPanel         = 0x0000000000100000,
                SunGlassPanel           = 0x0000000000200000,
                BlackGlassPanel         = 0x0000000000400000,
                WhiteGlassPanel         = 0x0000000000800000,
                PaintPanel              = 0x0000000001000000,
                PaintPanelMenu          = 0x0000000002000000,
                WindowSizeBar           = 0x0000000004000000,
                DataInformation         = 0x0000000008000000,
                FileList                = 0x0000000100000000,
                FindString              = 0x0000000200000000,
                DeletePathRequest       = 0x0000001000000000,
                RenamePathRequest       = 0x0000002000000000,
                NewFolderRequest        = 0x0000004000000000,
                PasteRequest            = 0x0000010000000000,
                OverWriteRequest        = 0x0000020000000000,
                NumberedRequest         = 0x0000040000000000,
                FreedomCursor           = 0x0000100000000000,
                FreedomUpDownPaper      = 0x0000200000000000,
                StartupOnetime          = 0x0001000000000000,
                FontBookOnetime         = 0x0002000000000000,
                MessageWindow           = 0x0004000000000000,
                RMainSetup              = 0x0010000000000000,
                RDeskImageSetup         = 0x0020000000000000,
                REditorSetup            = 0x0040000000000000,
                RDosSetup               = 0x0080000000000000,
                RInternetSetup          = 0x0100000000000000,
                RProgramSetup           = 0x0200000000000000,
                RChatSetup              = 0x0400000000000000,
                NoMenu                  = 0x2000000000000000,
                CloseDesk               = 0x4000000000000000,   // isize MAXIMUM : enum MAXIMUM
}

impl Into<usize> for BasicMode {
    fn into(self) -> usize {
        self as usize
    }
}

#[rustfmt::skip]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum SubMode {
    #[default]  SubModeNone             = 0x0000000000000000,
                RChatBasic              = 0x0000000000000001,
                RChatNormal             = 0x0000000000000002,
                RChatOpenaiStartup      = 0x0000000000000004,
                RChatGeminiStartup      = 0x0000000000000008,
                RDeskLeftPanel          = 0x0000000000000100,
                RDeskRightPanel         = 0x0000000000000200,
                RDeskUriInput           = 0x0000000000000400,
                RChatSystemMaster       = 0x0000000000000800,
                REditorLeftPanel        = 0x0000000000001000,
                REditorLeftCounter      = 0x0000000000002000,
                REditorRightPanel       = 0x0000000000004000,
                LinkingMicroView        = 0x0000000000008000,
                RDosLeftPanel           = 0x0000000000010000,
                RDosLeftCounter         = 0x0000000000020000,
                RDosRightPanel          = 0x0000000000040000,
                RChatLeftPanel          = 0x0000000000100000,
                RChatLeftCounter        = 0x0000000000200000,
                RChatRightPanel         = 0x0000000000400000,
                RDosCommandSetup        = 0x0000000001000000,
                RChatCommandSetup       = 0x0000000002000000,
                REditorCommandSetup     = 0x0000000004000000,
                RequestControl          = 0x0000000008000000,
                RSoundRecord            = 0x0000000010000000,
                RAudioToTextRecord      = 0x0000000020000000,
                RAudioToTextAuto        = 0x0000000040000000,
                RTextToSpeechAuto       = 0x0000000080000000,
                RSoundPlay              = 0x0000000100000000,
                RSpeechGooglePlay       = 0x0000000200000000,
                RTextToSpeechSoundPlay  = 0x0000000400000000,
                RAudioToTextSoundPlay   = 0x0000000800000000,
                RTextToSpeechOpenai     = 0x0000001000000000,
                RAudioToTextOpenai      = 0x0000002000000000,
                AudioSpeechConfig       = 0x0000008000000000,
                REditorFileNew          = 0x0000010000000000,
                REditorFileOpen         = 0x0000020000000000,
                REditorFileOpenApply    = 0x0000040000000000,
                REditorFileSave         = 0x0000080000000000,
                REditorFileSaveAs       = 0x0000100000000000,
                RImageFileSaveAs        = 0x0000200000000000,
                ClearUndoerReditor      = 0x0000400000000000,                
                RMasterEtc              = 0x0001000000000000,
                RDevSpeech              = 0x0002000000000000,
                RDevImage               = 0x0004000000000000,
                RDevGemini              = 0x0008000000000000,
                Rbishop                 = 0x0010000000000000,
                Rrc4plus                = 0x0020000000000000,
                RDevSpeechGoogle        = 0x0040000000000000,
                RCryptoMaster           = 0x0080000000000000,
                SettingDebug            = 0x0100000000000000,
                InspectionDebug         = 0x0200000000000000,
                MemoryDebug             = 0x0400000000000000,
//                DataInformation         = 0x0800000000000000,
                AdLeftPanel             = 0x1000000000000000,
                AdRightPanel            = 0x2000000000000000,
                AdDoublePanel           = 0x4000000000000000,   // isize MAXIMUM : enum MAXIMUM
}

impl Into<usize> for SubMode {
    fn into(self) -> usize {
        self as usize
    }
}

#[rustfmt::skip]
pub enum DosColor {
                BLACK           = 0x000000FF,   // r:0, g:0, b:0, a:255
                BLUE            = 0x0000FFFF,   // r:0, g:0, b:255, a:255
                GREEN           = 0x00AA00FF,   // r:0, g:170, b:0, a:255
                CYAN            = 0x009696FF,   // r:0, g:150, b:150, a:255
                RED             = 0xFF0000FF,   // r:255, g:0, b:0, a:255
                MAGENTA         = 0xCC00DCFF,   // r:204, g:0, b:220, a:255
                BROWN           = 0x952A2AFF,   // r:149, g:42, b:42, a:255
                LIGHTGRAY       = 0xDCDCDCFF,   // r:220, g:220, b:220, a:255
                GRAY            = 0xA0A0A0FF,   // r:160, g:160, b:160, a:255
                LIGHTBLUE       = 0x0082FFFF,   // r:0, g:130, b:255, a:255
                LIGHTGREEN      = 0x00FF00FF,   // r:0, g:255, b:0, a:255
                LIGHTCYAN       = 0x5AFFFFFF,   // r:90, g:255, b:255, a:255
                LIGHTRED        = 0xFF6060FF,   // r:255, g:96, b:96, a:255
                LIGHTMAGENTA    = 0xFF5AFFFF,   // r:255, g:90, b:255, a:255
                LIGHTYELLOW     = 0xFFFF00FF,   // r:255, g:255, b:0, a:255
                WHITE           = 0xFFFFFFFF,   // r:255, g:255, b:255, a:255
}

impl DosColor {
    pub fn as_color32(color_attr: u16) -> egui::Color32 {
        let color_value = match color_attr {
            x if x == 0 => DosColor::BLACK as u32,
            x if x == 1 => DosColor::BLUE as u32,
            x if x == 2 => DosColor::GREEN as u32,
            x if x == 3 => DosColor::CYAN as u32,
            x if x == 4 => DosColor::RED as u32,
            x if x == 5 => DosColor::MAGENTA as u32,
            x if x == 6 => DosColor::BROWN as u32,
            x if x == 7 => DosColor::LIGHTGRAY as u32,
            x if x == 8 => DosColor::GRAY as u32,
            x if x == 9 => DosColor::LIGHTBLUE as u32,
            x if x == 10 => DosColor::LIGHTGREEN as u32,
            x if x == 11 => DosColor::LIGHTCYAN as u32,
            x if x == 12 => DosColor::LIGHTRED as u32,
            x if x == 13 => DosColor::LIGHTMAGENTA as u32,
            x if x == 14 => DosColor::LIGHTYELLOW as u32,
            x if x == 15 => DosColor::WHITE as u32,
            _ => DosColor::RED as u32,
        };

        egui::Color32::from_rgba_premultiplied(
            ((color_value >> 24) & 0xFF) as u8,
            ((color_value >> 16) & 0xFF) as u8,
            ((color_value >> 8) & 0xFF) as u8,
            ((color_value) & 0xFF) as u8,
        )
    }
}

// ----------------------------------------------------------------------------
//
// MAIN ()
//
// ----------------------------------------------------------------------------

fn main() -> Result<(), eframe::Error> {
    // fn main() {

    let app_name = R_DESK_NAME;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]) // wide enough for the drag-drop overlay text
            .with_min_inner_size([400.0, 300.0])
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_transparent(true) // To have rounded corners we need transparency
            .with_icon(rdesk_icon())
            .with_drag_and_drop(true),
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
        centered: true,
        ..Default::default()
    };

    /*
        let mut app = bevy::app::App::new();
        app.add_plugins(bevy::DefaultPlugins)
            .add_plugins(bevy_egui::EguiPlugin)
            .init_resource::<SharedUiState>()
            .add_systems(bevy::app::Startup, load_assets_system)
    //        .add_systems(bevy::app::Startup, create_new_window_system)
            .add_systems(bevy::app::Update, ui_first_window_system);
    //        .add_systems(bevy::app::Update, ui_second_window_system);
        app.run();
    */

    eframe::run_native(
        app_name,
        options.clone(),
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Content::new(cc)))
        }),
    )
}

// ----------------------------------------------------------------------------
/*
use bevy::{
    //    prelude::*,
        ecs::system::*,
        window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
    };

    #[derive(Default, Resource)]
    struct SharedUiState {
        shared_input: String,
    }

    #[derive(Default)]
    struct UiState {
        input: String,
    }

    #[derive(Resource)]
    struct Images {
        bevy_icon: bevy::asset::Handle<bevy::render::texture::Image>,
    }

// ----------------------------------------------------------------------------

fn load_assets_system(mut commands: Commands, assets: Res<bevy::asset::AssetServer>) {
    commands.insert_resource(Images {
        bevy_icon: assets.load("icon.png"),
    });
}
// ----------------------------------------------------------------------------

fn ui_first_window_system(
    mut egui_user_textures: ResMut<bevy_egui::EguiUserTextures>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
    mut egui_ctx: Query<&mut bevy_egui::EguiContext, bevy::ecs::query::With<bevy::window::PrimaryWindow>>,
) {
    let bevy_texture_id = egui_user_textures.add_image(images.bevy_icon.clone_weak());
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    egui::Window::new("First Window")
    .vscroll(true)
    .show(ctx.get_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.input);
            });
            ui.horizontal(|ui| {
                ui.label("Shared input: ");
                ui.text_edit_singleline(&mut shared_ui_state.shared_input);
            });

            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                bevy_texture_id,
                [256.0, 256.0],
            )));

            if ui.button("Quit").clicked() {
                ui.close_menu();
            }
    });
}
*/

// ----------------------------------------------------------------------------

impl Content {
    // ------------------------------------------------------------------------

    //    pub fn new(cc: &eframe::CreationContext<'_>, _options: eframe::NativeOptions) -> Content {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Content {
        let mut content = Content {
            screen: Self::screen_new(),
            rdesk_exe_path: String::new(),
            rdesk_start_path: String::new(),
            rdesk_path: String::new(),
            rdesk_crypto_path: String::new(),
            rdesk_crypto_user_path: String::new(),
            rdesk_crypto_user_temp_path: String::new(),
            rdesk: PromptCommand::default(),
            rdesk_ccursor: Default::default(),
            savetime_started_limit: std::time::Instant::now(),
            check_rdesk_exist: 0,
            limit_login_seconds: LIMIT_LOGIN_SECONDS,
            limit_create_id_seconds: LIMIT_CREATE_ID_SECONDS,
            savetime_filetree: std::time::Instant::now(),
            savetime_freecursor: std::time::Instant::now(),
            savetime_dos: std::time::Instant::now(),
            savetime_messagewindow: std::time::Instant::now(),
            rdesk_id: String::new(),
            rdesk_password: String::new(),
            is_system_master: false,
            crypto: rcrypto::crypto_new(),
            bishop: Bishop::new("BBishop".to_string(), 9, 17),
            rc4: RC4Plus::new("".as_bytes(), "".as_bytes()),
            text_speech: Default::default(),
            audio_text: Default::default(),
            google_speech: Default::default(),
            sound: Default::default(),
            mictest: Default::default(),
            frame_history: FrameHistory::default(),
            //            lang_store: LangStore::new(),
            console_name: String::new(),
            console: ConsoleScreen::new(),
            dos: Default::default(),
            rdos: PromptCommand::default(),
            cmd_command: std::process::Command::new("cmd"),
            cmd_child_vec: Vec::new(),
            cmd_command_vec: Vec::new(),
            chat_text: Default::default(),
            chat_prompt: String::new(),
            chat_is_multiline: true,
            rchat: PromptCommand::default(),
            chat_tree: Vec::new(),
            chat: Self::chat_new(),
            o_chat: Self::o_chat_new(),
            g_chat: Self::g_chat_new(),
            o_image: Self::o_chat_new(),
            openai: Default::default(),
            gemini: Default::default(),
            editor: Default::default(),
            file_tree: Vec::new(),
            file_tree_immediately: true,
            filepannel: FilePannel::default(),
            selected_path: Default::default(),
            path_right_click: Default::default(),
            path_paste: Default::default(),
            path_copymove: Default::default(),
            encrypt_copymove: Default::default(),
            decrypt_copymove: Default::default(),
            input_rename: Default::default(),
            input_newfolder: Default::default(),
            reditor_fileopen: Default::default(),
            reditor_filesaveas: Default::default(),
            rimage_filesaveas: Default::default(),
            fontbook: FontBook::default(),
            rimage: Default::default(),
            ad: Self::r_ad_new(),
            painting: rimage::Painting::default(),
            browser_path: String::new(),
            status_doswin_hide: false,
            viewport_master_etc: Default::default(),
            viewport_master_etc_immediate: Default::default(),
            viewport_dev_speech_immediate: Default::default(),
            viewport_dev_speech_google_immediate: Default::default(),
            close_requested: false,
            close_cancelable: true,
        };

        content.rdos = PromptCommand {
            command: String::new(),
            change_command: false,
            end_of_cursor: false,
            onetime_focus: true,
            twotime_focus: false,
            is_prompt_before: true,
            is_prompt_on: true,
            change_prompt: true,
            check_processing: false,
            command_history: Vec::new(),
            command_total: 0,
            command_pos: 0,
        };

        content.screen.always_on_top = true;
        content.screen.window_size = egui::Vec2::new(1280.0, 720.0);
        content.screen.window_size_min = egui::Vec2::new(400.0, 300.0);
        content.screen.dark_mode = true;

        content.screen.clearglass_value = 0;
        content.screen.sunglass_value = 200;
        content.screen.blackglass_value = 0;
        content.screen.whiteglass_value = 255;
        content.screen.button_padding_x = 6.0;
        content.screen.button_padding_y = 3.0;
        content.screen.item_spacing_x = 3.0;
        content.screen.item_spacing_y = 3.0;

        content.screen.font_adjust = 1.40;
        content.screen.font_size_large = 15.0;
        content.screen.font_size_normal = 13.0;
        content.screen.font_size_small = 11.0;
        content.screen.font_size_desk = 15.0;
        content.screen.font_size_roompannel = 13.0;
        content.screen.font_size_microview = 1.5;
        content.screen.font_size_filepannel = 13.0;
        content.screen.font_scale = 1.25;

        content.screen.leftpanel_width_default = 200.0;
        content.screen.leftpanel_width = content.screen.leftpanel_width_default;
        content.screen.leftpanel_width_min = 100.0;
        content.screen.rightpanel_width_default = 200.0;
        content.screen.rightpanel_width = content.screen.rightpanel_width_default;
        content.screen.rightpanel_width_min = 100.0; // Minimum unbreakable size = 85.0

        content.rimage.image_scale = 1.0;
        set_o_image_request_default(&mut content.o_image);

        set_openai_url_path_vec(&mut content.openai);
        set_gemini_url_path_vec(&mut content.gemini);
        // get_g_chat_model(&mut content);
        // content.g_chat.ehttp.request_url = content.gemini.url_path_vec[1].clone();

        //        let mut rng = rand::thread_rng();
        //        let desk_num: usize = rand::Rng::gen_range(&mut rng, 0..1_000_000);
        let desk_pid = std::process::id();
        content.console_name = format!("{} {}", R_DESK_NAME, desk_pid);

        if let Some(exe_path) = std::env::current_exe().ok() {
            if let Some(parent_dir) = exe_path.parent() {
                content.rdesk_exe_path = parent_dir.display().to_string();
            }
        }

        if let Some(current_path) = std::env::current_dir().ok() {
            content.rdesk_start_path = current_path.display().to_string();
            content.selected_path.path = content.rdesk_start_path.clone();
        }

        content.screen.basic_mode = 0; // Clear
        basicmode_on(&mut content, BasicMode::RDesk);

        LangStore::set_default_langfile_path(&mut content);
        //        content.lang_store = LangStore::new();
        content.screen.lang_store.load_file_lang_store_basic();
        Self::apply_new_lang_store(&mut content, &cc.egui_ctx);
        set_default_file_name_auto(&mut content);

        let home_drive: String = std::env::var("HOMEDRIVE").unwrap_or("C:".to_string());
        content.rdesk_path.push_str(&home_drive);
        content.rdesk_path.push_str(R_DESK_DIRECTORY_NAME);
        content.rdesk_crypto_path.push_str(&content.rdesk_path);
        content.rdesk_crypto_path.push_str(r#"\CRYPTO"#);
        content
            .rdesk_crypto_user_path
            .push_str(&content.rdesk_crypto_path);
        content.rdesk_crypto_user_path.push_str(R_DESK_USER_DEFAULT);
        let temp_name = r_crypto_filename_encrypt(&mut content, "TEMP".to_string());
        content
            .rdesk_crypto_user_temp_path
            .push_str(&content.rdesk_crypto_user_path);
        // content.rdesk_crypto_user_temp_path.push_str(r"\");
        add_backslash(&mut content.rdesk_crypto_user_temp_path);
        content.rdesk_crypto_user_temp_path.push_str(&temp_name);
        /*
                content.rdesk.command = get_text(&mut content, "image-uri-startup:setup");
                if !content.rdesk.command.trim().is_empty() {
                    content.rdesk.change_command = true;
                }
                else {
                    content.rdesk.command.clear();
                    contentrimage.rimage.image_onetime_internet = true;
                    contentrimage.rimage.image_onetime_file = true;
                    contentrimage.rimage.image_onetime_random = true;
                }
        */
        content.sound.audio.volume = 0.5;
        content.sound.audio.speed = 1.0;

        content.mictest.audio.volume = 0.5;
        content.mictest.audio.speed = 1.0;

        content.audio_text.audio.volume = 0.5;
        content.audio_text.audio.speed = 1.0;
        content.audio_text.audio_request.language = Some("ko".to_string());

        content.text_speech.audio.volume = 0.5;
        content.text_speech.audio.speed = 1.0;

        content.google_speech.audio.volume = 0.5;
        content.google_speech.audio.speed = 1.0;
        content.google_speech.speech_language = Some("ko".to_string());
        content.google_speech.speech_using_file = false;
        content.google_speech.speech_dat_path = content.rdesk_exe_path.clone();
        // content.google_speech.speech_dat_path.push_str(r"\");
        add_backslash(&mut content.google_speech.speech_dat_path);
        content
            .google_speech
            .speech_dat_path
            .push_str("rdesk-speech-ko.dat");
        content.google_speech.google_client = "tw-ob".to_string();

        set_openai_request_default(&mut content.o_chat, &mut content.chat);
        set_gemini_request_default(&mut content.g_chat, &mut content.chat);

        content.g_chat.is_ongoing_talk = true;
        content.o_chat.is_ongoing_talk = false;

        unsafe {
            winapi::um::winuser::EnumWindows(Some(Content::check_rdesk_enum_windows_callback), 0);
        }
        content.check_rdesk_exist = RDESK_COUNT.load(std::sync::atomic::Ordering::SeqCst);
        for _ in 0..content.check_rdesk_exist {
            content.limit_login_seconds *= 2;
        }
        for _ in 0..content.check_rdesk_exist {
            content.limit_create_id_seconds *= 2;
        }

        return content;
    }

    // ------------------------------------------------------------------------

    pub fn screen_new() -> ScreenData {
        let screen: ScreenData = ScreenData {
            lang_store: LangStore::new(),
            ..Default::default()
        };

        return screen;
    }

    // ------------------------------------------------------------------------

    pub fn apply_new_lang_store(content: &mut Content, ctx: &egui::Context) {
        let mut home_path = std::env::var("HOMEPATH").unwrap_or("\\Users\\user".to_string());
        home_path.push_str("\\Pictures");
        add_setup_str(content, "image-path-default:setup", &home_path);

        content.rimage.image_path = get_text(content, "image-path-1:setup");
        content.browser_path = get_text(content, "browser-path:setup");

        content.rimage.image_address = get_text(content, "image-url:setup");
        get_image_list(content);

        content.screen.font_scale = get_text(content, "font-scale:setup")
            .parse()
            .unwrap_or(1.25);
        content.screen.font_y_offset_factor = get_text(content, "font-y-offset-factor:setup")
            .parse()
            .unwrap_or(-0.27);
        content.screen.font_y_offset = get_text(content, "font-y-offset:setup")
            .parse()
            .unwrap_or(0.0);

        content.screen.clearglass_value = get_text(content, "clearglass-value:setup")
            .parse()
            .unwrap_or(0);
        content.screen.sunglass_value = get_text(content, "sunglass-value:setup")
            .parse()
            .unwrap_or(200);
        content.screen.blackglass_value = get_text(content, "blackglass-value:setup")
            .parse()
            .unwrap_or(0);
        content.screen.whiteglass_value = get_text(content, "whiteglass-value:setup")
            .parse()
            .unwrap_or(255);

        CodeTheme::set_themecolor_dark_vec_default(content);
        CodeTheme::set_themecolor_light_vec_default(content);
        CodeTheme::set_themecolor_dark(content, Theme::MainMenu);
        CodeTheme::set_themecolor_light(content, Theme::MainMenu);

        let preload_fontfilename = "NanumSquareNeoOTF-aLt.otf".to_string();
        content.screen.preload_fontfilename = preload_fontfilename.clone();
        apply_font_file_setup(content, ctx);
        apply_font_size_setup(content);
        apply_botton_padding_setup(content);
        content.filepannel.font_id = FontId::monospace(content.screen.font_size_filepannel);

        //        content.editor.row_height = content.editor.font_size + 1.6;         // temporary row_height
        //        set_default_file_name_auto(content);

        apply_window_size_setup(content);

        basicmode_new(content);
        basicmode_on(content, BasicMode::StartupOnetime);

        submode_new(content);

        r_crypto_startup_apikey_openai(content);
        r_crypto_startup_apikey_gemini(content);

        apply_r_login_chat_setup(content);
        apply_r_dos_prompt_before_setup(content);
    }

    // ------------------------------------------------------------------------
    /*
        pub fn ehttp_new() -> Ehttp {
            let ehttp = Ehttp {
                start_trigger: false,
                method: Method::Get,
                streaming: true,
                inprogress: false,
                download: std::sync::Arc::new(std::sync::Mutex::new(Download::None)),
                request_url: String::new(),
                request_text: String::new(),
                response_bytes: Vec::new(),
                response_url: String::new(),
                response_text: String::new(),
                content_type: String::new(),
    //            num_bytes: 0,
            };

            ehttp
        }
    */
    // ------------------------------------------------------------------------

    pub fn chat_new() -> Chat {
        let chat = Chat {
            font_id_roompannel: FontId::monospace(13.0),
            is_joined_chatroom: false,
            joined_chatroom: Default::default(),
            selected_chatroom: Default::default(),
            visible_chat_login_setup: false,
            chatroom_user_name: String::new(),
            chatroom_user_sex: String::new(),
            chatroom_user_age: String::new(),
            chatroom_assist_name: String::new(),
            chatroom_assist_sex: String::new(),
            chatroom_assist_age: String::new(),
            language: String::new(),
            system_message: String::new(),
        };

        chat
    }

    pub fn o_chat_new() -> OpenaiChat {
        let mut chat = OpenaiChat {
            ehttp: Default::default(), // Self::ehttp_new(),
            num_received: 0,
            view_response_all: true,
            auto_hidden_clear: true,
            is_ongoing_talk: false,
            is_not_talk: false,
            // is_joined_chatroom: false,
            // joined_chatroom: Default::default(),
            // selected_chatroom: Default::default(),
            system_message: String::new(),
            openai_apikey_user: String::new(),
            openai_model: "gpt-3.5-turbo".to_string(),
            completion_request: Default::default(),
            //            completion_response: Default::default(),
            error_response: Default::default(),
            finish_reason: String::new(),
            hidden_text: String::new(),
            input_text: String::new(),
            output_text: String::new(),
            image_request: Default::default(),
            revised_prompt: None,
            b64_json: Default::default(),
            show_image_onetime: false,
            history: Default::default(),
        };

        chat.ehttp.request_url = "https://api.openai.com/v1/chat/completions".to_string();

        chat
    }

    pub fn g_chat_new() -> GeminiChat {
        let mut chat = GeminiChat {
            ehttp: Default::default(), // Self::ehttp_new(),
            num_received: 0,
            view_response_all: true,
            auto_hidden_clear: true,
            is_ongoing_talk: false,
            is_not_talk: false,
            // is_joined_chatroom: false,
            // joined_chatroom: Default::default(),
            // selected_chatroom: Default::default(),
            system_message: String::new(),
            finish_reason: String::new(),
            hidden_text: String::new(),
            input_text: String::new(),
            output_text: String::new(),
            vision_request_path: String::new(),
            gemini_apikey_user: String::new(),
            gemini_request: Default::default(),
            gemini_response: Default::default(),
            gemini_error_response: Default::default(),
            history: Default::default(),
        };

        chat.ehttp.request_url = make_gemini_url_path("models/gemini-pro", "", false);

        chat
    }

    // ------------------------------------------------------------------------
    /*
        pub fn ai_server_new() -> AiServer {
            let mut ai_server = AiServer {
                model_text: String::new(),
                model_vec: Vec::new(),
                url_path_vec: Vec::new(),
                openai_apikey: String::new(),
                gemini_apikey: String::new(),
            };

            ai_server
        }
    */
    // ------------------------------------------------------------------------

    pub fn r_ad_new() -> AD {
        let ad = AD {
            ehttp: Default::default(), //  Self::ehttp_new(),
            started_time: 0.0,
            reload_count: 0,
            duration_count: 0,
            frame_count: 0,
            url: String::new(),
            uri_before: String::new(),
            click_url: String::new(),
            //            click_url_before: String::new(),
            set_file: String::new(),
            setup: LangStore::ad_new(),
            max_num: 0,
            display_num: 0,
            display_frame_num: 0,
            ad_image_url: Vec::new(),
            ad_image_click_url: Vec::new(),
            onetime_setup_read: true,
            onetime_read: true,
            width: 200.0,
            height: 150.0,
            image_data: Default::default(),
            is_ad_request_change: false,
        };

        ad
    }

    // ------------------------------------------------------------------------

    unsafe extern "system" fn check_rdesk_enum_windows_callback(
        window_handle: winapi::shared::windef::HWND,
        _result: winapi::shared::minwindef::LPARAM,
    ) -> i32 {
        let mut window_title: [u16; 256] = [0; 256];

        winapi::um::winuser::GetWindowTextW(window_handle, window_title.as_mut_ptr(), 255);
        if winapi::um::winuser::IsWindowVisible(window_handle) != 0 && window_title[0] != 0 {
            let title = String::from_utf16_lossy(&window_title);
            if title.contains(R_DESK_NAME) {
                RDESK_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        return 1;
    }

    // ------------------------------------------------------------------------
    // imple Content End.
    // ------------------------------------------------------------------------
} // impl Content

// ----------------------------------------------------------------------------
// BasicMode Function
// ----------------------------------------------------------------------------

pub fn basicmode_new(content: &mut Content) {
    //    content.basic_mode = 0;                                 // Clear
    //    basicmode_on(content, BasicMode::RDesk);

    // R-DESK Setup Mode
    apply_rdesk_mode_setup(content);
    apply_rdesk_left_right_setup(content);
    //    submode_on(content, SubMode::RDeskUriInput);

    // Panel Setup Mode
    apply_r_panel_setup_new(content);

    // R-EDITOR Setup Mode
    let key_text = "r-editor-mode:setup";
    let setup: String = get_text(content, key_text);
    let normal_mode = get_text(content, "r-editor-normal:button");
    if setup == normal_mode {
        r_editor_sub_toggle(content, BasicMode::REditorNormal);
    } else {
        r_editor_sub_toggle(content, BasicMode::REditorBasic);
    }

    // R-EDITOR Sub Setup Mode
    apply_editor_fuction_setup(content);

    // R-DOS Default Mode
    apply_r_dos_mode_setup(content);

    // R-CHAT Default Mode
    apply_r_chat_mode_setup(content);

    // FPS mode
    apply_fps_mode_setup(content);
}

// ----------------------------------------------------------------------------

pub fn submode_new(content: &mut Content) {
    apply_r_dos_panel_setup(content);
    apply_r_chat_panel_setup(content);

    // R-CHAT Default Mode
    submode_on(content, SubMode::RChatBasic);
    // submode_on(content, SubMode::RChatStartup); // Absolutely Necessary
    submode_on(content, SubMode::RChatLeftPanel);
    submode_on(content, SubMode::RChatRightPanel);

    submode_on(content, SubMode::RequestControl);

    submode_on(content, SubMode::LinkingMicroView);

    // submode_on(content, SubMode::RTextToSpeechAuto);

    //    r_ad_toggle(content, SubMode::AdDoublePanel);
    apply_r_ad_setup(content);
}

// ----------------------------------------------------------------------------

#[inline]
pub fn is_mode<T: Into<usize>>(save_mode: usize, mode: T) -> bool {
    (save_mode & mode.into()) != 0 as usize
}

#[inline]
pub fn mode_on<T: Into<usize>>(save_mode: &mut usize, mode: T) {
    *save_mode |= mode.into();
}

#[inline]
pub fn mode_off<T: Into<usize>>(save_mode: &mut usize, mode: T) {
    *save_mode &= !mode.into();
}

#[inline]
pub fn mode_toggle<T: Into<usize> + Copy>(save_mode: &mut usize, mode: T) {
    if is_mode(*save_mode, mode) {
        mode_off(save_mode, mode);
    } else {
        mode_on(save_mode, mode);
    }
}

// ----------------------------------------------------------------------------

#[inline]
pub fn is_basicmode(content: &Content, mode: BasicMode) -> bool {
    // (content.screen.basic_mode & mode as usize) != BasicMode::BasicModeNone as usize
    is_mode(content.screen.basic_mode, mode)
}

#[inline]
pub fn basicmode_on(content: &mut Content, mode: BasicMode) {
    // content.screen.basic_mode |= mode as usize;
    mode_on(&mut content.screen.basic_mode, mode);
}

#[inline]
pub fn basicmode_off(content: &mut Content, mode: BasicMode) {
    // content.screen.basic_mode &= !(mode as usize);
    mode_off(&mut content.screen.basic_mode, mode);
}

#[inline]
pub fn basicmode_toggle(content: &mut Content, mode: BasicMode) {
    /*
    if is_basicmode(content, mode) {
        basicmode_off(content, mode);
    } else {
        basicmode_on(content, mode);
    }
    */
    mode_toggle(&mut content.screen.basic_mode, mode);
}

// ----------------------------------------------------------------------------

#[inline]
pub fn is_submode(content: &Content, mode: SubMode) -> bool {
    // (content.screen.sub_mode & mode as usize) != SubMode::SubModeNone as usize
    is_mode(content.screen.sub_mode, mode)
}

#[inline]
pub fn submode_on(content: &mut Content, mode: SubMode) {
    // content.screen.sub_mode |= mode as usize;
    mode_on(&mut content.screen.sub_mode, mode);
}

#[inline]
pub fn submode_off(content: &mut Content, mode: SubMode) {
    // content.screen.sub_mode &= !(mode as usize);
    mode_off(&mut content.screen.sub_mode, mode);
}

#[inline]
pub fn submode_toggle(content: &mut Content, mode: SubMode) {
    /*
    if is_submode(content, mode) {
        submode_off(content, mode);
    } else {
        submode_on(content, mode);
    }
    */
    mode_toggle(&mut content.screen.sub_mode, mode);
}

// ----------------------------------------------------------------------------
/*
#[inline]
pub fn is_screen_submode(screen: &ScreenData, mode: SubMode) -> bool {
    // (screen.sub_mode & mode as usize) != SubMode::SubModeNone as usize
    is_mode(screen.sub_mode, mode)
}

#[inline]
pub fn screen_submode_on(screen: &mut ScreenData, mode: SubMode) {
    // screen.sub_mode |= mode as usize;
    mode_on(&mut screen.sub_mode, mode);
}

#[inline]
pub fn screen_submode_off(screen: &mut ScreenData, mode: SubMode) {
    // screen.sub_mode &= !(mode as usize);
    mode_off(&mut screen.sub_mode, mode);
}

#[inline]
pub fn screen_submode_toggle(screen: &mut ScreenData, mode: SubMode) {
    /*
    if is_screen_submode(screen, mode) {
        screen_submode_off(screen, mode);
    } else {
        screen_submode_on(screen, mode);
    }
    */
    mode_toggle(&mut screen.sub_mode, mode);
}
*/
// ----------------------------------------------------------------------------

pub fn apply_r_panel_setup_new(content: &mut Content) {
    let setup = get_text(content, "r-glass:setup");
    if setup == get_text(content, "r-clearglass:menu") {
        basicmode_on(content, BasicMode::ClearGlassPanel);
    } else if setup == get_text(content, "r-blackglass:menu") {
        basicmode_on(content, BasicMode::BlackGlassPanel);
    } else if setup == get_text(content, "r-whiteglass:menu") {
        basicmode_on(content, BasicMode::WhiteGlassPanel);
    } else {
        basicmode_on(content, BasicMode::SunGlassPanel);
    }
}

// ----------------------------------------------------------------------------

pub fn apply_r_panel_setup(content: &mut Content, ui: &mut egui::Ui) {
    let setup = get_text(content, "r-glass:setup");
    if setup == get_text(content, "r-clearglass:menu") {
        glasspannel_toggle(content, ui, BasicMode::ClearGlassPanel);
    } else if setup == get_text(content, "r-blackglass:menu") {
        glasspannel_toggle(content, ui, BasicMode::BlackGlassPanel);
    } else if setup == get_text(content, "r-whiteglass:menu") {
        glasspannel_toggle(content, ui, BasicMode::WhiteGlassPanel);
    } else {
        glasspannel_toggle(content, ui, BasicMode::SunGlassPanel);
    }
}

// ----------------------------------------------------------------------------

pub fn glasspannel_toggle(content: &mut Content, ui: &mut egui::Ui, mode: BasicMode) {
    basicmode_off(content, BasicMode::ClearGlassPanel);
    basicmode_off(content, BasicMode::SunGlassPanel);
    basicmode_off(content, BasicMode::BlackGlassPanel);
    basicmode_off(content, BasicMode::WhiteGlassPanel);

    basicmode_on(content, mode);
    if mode == BasicMode::WhiteGlassPanel {
        set_light_mode(content, ui);
    } else {
        set_dark_mode(content, ui);
    }
}

// ----------------------------------------------------------------------------

pub fn glasspannel_toggle_next(content: &mut Content, ui: &mut egui::Ui) {
    if is_basicmode(content, BasicMode::ClearGlassPanel) {
        basicmode_off(content, BasicMode::ClearGlassPanel);
        basicmode_on(content, BasicMode::SunGlassPanel);
    } else if is_basicmode(content, BasicMode::SunGlassPanel) {
        basicmode_off(content, BasicMode::SunGlassPanel);
        basicmode_on(content, BasicMode::BlackGlassPanel);
    } else if is_basicmode(content, BasicMode::BlackGlassPanel) {
        basicmode_off(content, BasicMode::BlackGlassPanel);
        basicmode_on(content, BasicMode::WhiteGlassPanel);
    } else if is_basicmode(content, BasicMode::WhiteGlassPanel) {
        basicmode_off(content, BasicMode::WhiteGlassPanel);
        basicmode_on(content, BasicMode::ClearGlassPanel);
    }

    if is_basicmode(content, BasicMode::WhiteGlassPanel) {
        set_light_mode(content, ui);
    } else {
        set_dark_mode(content, ui);
    }
}

// ----------------------------------------------------------------------------
/*
pub fn glasspannel_defualt_on(content: &mut Content) {

    if  !is_basicmode(content, BasicMode::ClearGlassPanel) &&
        !is_basicmode(content, BasicMode::SunGlassPanel) &&
        !is_basicmode(content, BasicMode::BlackGlassPanel) &&
        !is_basicmode(content, BasicMode::WhiteGlassPanel)
    {
        glasspannel_toggle(content, BasicMode::SunGlassPanel);
    }

}
*/
// ----------------------------------------------------------------------------

pub fn paintpanel_toggle(content: &mut Content, mode: BasicMode) {
    if mode != BasicMode::PaintPanel && mode != BasicMode::PaintPanelMenu {
        return;
    }

    if !is_basicmode(content, BasicMode::PaintPanel) {
        basicmode_off(content, BasicMode::RDesk);
        basicmode_off(content, BasicMode::REditor);
        basicmode_off(content, BasicMode::RDos);
        basicmode_off(content, BasicMode::RChat);
    }

    if !is_basicmode(content, BasicMode::PaintPanel)
        && !is_basicmode(content, BasicMode::PaintPanelMenu)
    {
        basicmode_on(content, BasicMode::PaintPanel);
        basicmode_on(content, BasicMode::PaintPanelMenu);
    } else if is_basicmode(content, BasicMode::PaintPanelMenu) {
        basicmode_on(content, BasicMode::PaintPanel);
        basicmode_off(content, BasicMode::PaintPanelMenu);
    } else {
        basicmode_off(content, BasicMode::PaintPanel);
        basicmode_off(content, BasicMode::PaintPanelMenu);
    }
}

// ----------------------------------------------------------------------------

pub fn rdesk_sub_toggle(content: &mut Content, mode: BasicMode) {
    if mode == BasicMode::RDeskInternetImage {
        basicmode_on(content, BasicMode::RDeskInternetImage);
        basicmode_off(content, BasicMode::RDeskFileImage);
    } else if mode == BasicMode::RDeskFileImage {
        basicmode_off(content, BasicMode::RDeskInternetImage);
        basicmode_on(content, BasicMode::RDeskFileImage);
    }
}

// ----------------------------------------------------------------------------

pub fn r_editor_sub_toggle(content: &mut Content, mode: BasicMode) {
    if mode == BasicMode::REditorBasic {
        basicmode_on(content, BasicMode::REditorBasic);
        basicmode_off(content, BasicMode::REditorNormal);
    } else if mode == BasicMode::REditorNormal {
        basicmode_on(content, BasicMode::REditorNormal);
        basicmode_off(content, BasicMode::REditorBasic);
    }
}

// ----------------------------------------------------------------------------

pub fn r_dos_sub_toggle(content: &mut Content, mode: BasicMode) {
    if mode == BasicMode::RDosBasic {
        basicmode_on(content, BasicMode::RDosBasic);
        basicmode_off(content, BasicMode::RDosNormal);
    //        content.dos.onetime_focus = true;
    } else if mode == BasicMode::RDosNormal {
        basicmode_on(content, BasicMode::RDosNormal);
        basicmode_off(content, BasicMode::RDosBasic);
        //        content.dos.onetime_focus = true;
    }
}

// ----------------------------------------------------------------------------

pub fn r_chat_sub_toggle(content: &mut Content, mode: SubMode) {
    if mode == SubMode::RChatBasic {
        submode_on(content, SubMode::RChatBasic);
        submode_off(content, SubMode::RChatNormal);
    //        content.rchat.onetime_focus = true;
    } else if mode == SubMode::RChatNormal {
        submode_on(content, SubMode::RChatNormal);
        submode_off(content, SubMode::RChatBasic);
        //        content.rchat.onetime_focus = true;
    }
}

// ----------------------------------------------------------------------------

pub fn mainmenu_on(content: &mut Content, mode: BasicMode) {
    if mode == BasicMode::RDesk
        || mode == BasicMode::REditor
        || mode == BasicMode::RDos
        || mode == BasicMode::RChat
        || mode == BasicMode::PaintPanel
    {
        basicmode_off(content, BasicMode::RDesk);
        basicmode_off(content, BasicMode::REditor);
        basicmode_off(content, BasicMode::RDos);
        basicmode_off(content, BasicMode::RChat);
        basicmode_off(content, BasicMode::PaintPanel);
        basicmode_off(content, BasicMode::PaintPanelMenu);
    }

    basicmode_on(content, mode);
}

// ----------------------------------------------------------------------------

pub fn mainmenu_toggle(content: &mut Content, mode: BasicMode) {
    //    let mode_on = !is_basicmode(content, mode);

    mainmenu_on(content, mode);

    /*
        if mode_on {
            basicmode_on(content, mode);
        }
        else {
            basicmode_off(content, mode);
        }
    */
}

// ----------------------------------------------------------------------------

pub fn r_ad_toggle(content: &mut Content, mode: SubMode) {
    if mode == SubMode::AdLeftPanel {
        submode_on(content, SubMode::AdLeftPanel);
        submode_off(content, SubMode::AdRightPanel);
        submode_off(content, SubMode::AdDoublePanel);
        // content.screen.leftpanel_width = 200.0;
        // content.screen.leftpanel_width_min = 100.0;
        // content.screen.rightpanel_width_min = 100.0;
    } else if mode == SubMode::AdRightPanel {
        submode_off(content, SubMode::AdLeftPanel);
        submode_on(content, SubMode::AdRightPanel);
        submode_off(content, SubMode::AdDoublePanel);
        // content.screen.rightpanel_width = 200.0;
        // content.screen.leftpanel_width_min = 100.0;
        // content.screen.rightpanel_width_min = 100.0;
    } else {
        submode_off(content, SubMode::AdLeftPanel);
        submode_off(content, SubMode::AdRightPanel);
        submode_on(content, SubMode::AdDoublePanel);
        // content.screen.leftpanel_width = 200.0;
        // content.screen.rightpanel_width = 200.0;
        // content.screen.leftpanel_width_min = 100.0;
        // content.screen.rightpanel_width_min = 100.0;
    }
}

// ----------------------------------------------------------------------------

pub fn exist_rightpanel(content: &mut Content) -> bool {
    let exist = (is_basicmode(content, BasicMode::RDesk)
        && is_submode(content, SubMode::RDeskRightPanel))
        || (is_basicmode(content, BasicMode::REditor)
            && is_submode(content, SubMode::REditorRightPanel))
        || (is_basicmode(content, BasicMode::RDos) && is_submode(content, SubMode::RDosRightPanel))
        || (is_basicmode(content, BasicMode::RChat)
            && is_submode(content, SubMode::RChatRightPanel));

    exist
}

// ----------------------------------------------------------------------------

pub fn exist_leftpanel(content: &mut Content) -> bool {
    let exist = (is_basicmode(content, BasicMode::RDesk)
        && is_submode(content, SubMode::RDeskLeftPanel))
        || (is_basicmode(content, BasicMode::REditor)
            && is_submode(content, SubMode::REditorLeftPanel))
        || (is_basicmode(content, BasicMode::RDos) && is_submode(content, SubMode::RDosLeftPanel))
        || (is_basicmode(content, BasicMode::RChat)
            && is_submode(content, SubMode::RChatLeftPanel));

    exist
}

// ----------------------------------------------------------------------------

fn access_time(save_time: &mut std::time::Instant, sec: f32) -> bool {
    let elapsed = save_time.elapsed();
    if elapsed >= std::time::Duration::from_secs_f32(sec) {
        *save_time = std::time::Instant::now();
        return true;
    }
    return false;
}

// ----------------------------------------------------------------------------

pub fn apply_font_size_setup(content: &mut Content) {
    let key_text = "font-size-small:setup";
    let setup: String = get_text(content, key_text);
    let font_size_small: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.font_size_small,
    };
    content.screen.font_size_small = font_size_small;

    let key_text = "font-size-normal:setup";
    let setup: String = get_text(content, key_text);
    let font_size_normal: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.font_size_normal,
    };
    content.screen.font_size_normal = font_size_normal;

    let key_text = "font-size-large:setup";
    let setup: String = get_text(content, key_text);
    let font_size_large: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.font_size_large,
    };
    content.screen.font_size_large = font_size_large;

    let key_text = "font-size-editor:setup";
    let setup: String = get_text(content, key_text);
    let font_size_editor: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.editor.font_size // default 15.0
        }
    };
    content.editor.font_size = font_size_editor;

    let key_text = "font-size-dos:setup";
    let setup: String = get_text(content, key_text);
    let font_size_dos: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.dos.font_size // default 15.0
        }
    };
    content.dos.font_size = font_size_dos;

    let key_text = "font-size-chat:setup";
    let setup: String = get_text(content, key_text);
    let font_size_chat: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.chat_text.font_size // default 15.0
        }
    };
    content.chat_text.font_size = font_size_chat;

    let key_text = "font-size-roompannel:setup";
    let setup: String = get_text(content, key_text);
    let font_size_roompannel: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.screen.font_size_roompannel // default 13.0
        }
    };
    content.screen.font_size_roompannel = font_size_roompannel;
    content.chat.font_id_roompannel = FontId::monospace(content.screen.font_size_roompannel);

    let key_text = "font-size-filepannel:setup";
    let setup: String = get_text(content, key_text);
    let font_size_filepannel: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.screen.font_size_filepannel // default 13.0
        }
    };
    content.screen.font_size_filepannel = font_size_filepannel;
    content.filepannel.font_id = FontId::monospace(content.screen.font_size_filepannel);

    let key_text = "md_tooltip_font_size:setup";
    let setup: String = get_text(content, key_text);
    let md_tooltip_font_size: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.filepannel.md_tooltip_font_size // default 13.0
        }
    };
    content.filepannel.md_tooltip_font_size = md_tooltip_font_size;
    content.filepannel.md_tooltip_font_id =
        egui::FontId::monospace(content.filepannel.md_tooltip_font_size);

    let key_text = "md_tooltip_width:setup";
    let setup: String = get_text(content, key_text);
    let md_tooltip_width: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.filepannel.md_tooltip_width // default 800.0
        }
    };
    content.filepannel.md_tooltip_width = md_tooltip_width;

    let key_text = "md_tooltip_height:setup";
    let setup: String = get_text(content, key_text);
    let md_tooltip_height: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.filepannel.md_tooltip_height // default 500.0
        }
    };
    content.filepannel.md_tooltip_height = md_tooltip_height;
}

// ----------------------------------------------------------------------------

pub fn apply_botton_padding_setup(content: &mut Content) {
    let key_text = "item_spacing_x:setup";
    let setup: String = get_text(content, key_text);
    let item_spacing_x: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.item_spacing_x,
    };
    content.screen.item_spacing_x = item_spacing_x;

    let key_text = "item_spacing_y:setup";
    let setup: String = get_text(content, key_text);
    let item_spacing_y: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.item_spacing_y,
    };
    content.screen.item_spacing_y = item_spacing_y;

    let key_text = "button_padding_x:setup";
    let setup: String = get_text(content, key_text);
    let button_padding_x: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.button_padding_x,
    };
    content.screen.button_padding_x = button_padding_x;

    let key_text = "button_padding_y:setup";
    let setup: String = get_text(content, key_text);
    let button_padding_y: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => content.screen.button_padding_y,
    };
    content.screen.button_padding_y = button_padding_y;
}

// ----------------------------------------------------------------------------
/*
pub fn apply_font_size_filepannel_setup(content: &mut Content) {

    let key_text = "font-size-filepannel:setup";
    let setup: String = get_text(content, key_text);
    let font_size_filepannel: f32 = match setup.parse() {
        Ok(value) => value,
        Err(_) => {
            content.font_size_filepannel                  // default 13.0
        }
    };

    content.font_size_filepannel = font_size_filepannel;
    content.filepannel.font_id = FontId::monospace( content.font_size_filepannel );
}
*/
// ----------------------------------------------------------------------------

pub fn apply_font_file_setup(content: &mut Content, ctx: &egui::Context) {
    let key_text = "font-filepath:setup";
    content.screen.font_filename = get_text(content, key_text);
    setup_file_fonts(content, ctx);
}

// ----------------------------------------------------------------------------
// setup Font : loaded Program Compile time.
// ----------------------------------------------------------------------------

pub fn setup_preload_fonts(content: &mut Content, ctx: &egui::Context) {
    content.screen.font_filename = content.screen.preload_fontfilename.clone(); // "NanumSquareNeoOTF-aLt.otf"
    get_font_list(content);

    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    content.screen.preload_fontfiledata = core::include_bytes!(
        "../NanumSquareNeoOTF-aLt.otf" //            "../D2Coding-Ver1.3.2-20180524-ligature.ttf"
                                       //            "../NanumSquareNeoOTF-aLt.otf"
                                       //            "../NanumBarunGothicLight.otf"
    );

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(content.screen.preload_fontfiledata).tweak(FontTweak {
            scale: content.screen.font_scale, // make it smaller or taller
            y_offset_factor: content.screen.font_y_offset_factor, // move it down(+) or up(-)
            y_offset: content.screen.font_y_offset,
            baseline_offset_factor: -0.0333, // makes the default fonts look more centered in buttons and such
        }),
    );

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    //        .insert(0, "my_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("my_font".to_owned());
    //        .insert(0, "my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

// ----------------------------------------------------------------------------
// setup File Font
// ----------------------------------------------------------------------------

pub fn setup_file_fonts(content: &mut Content, ctx: &egui::Context) {
    let font_filename = content.screen.font_filename.clone();

    if font_filename == content.screen.preload_fontfilename {
        // --------------------------------------
        // Default Fonts
        // --------------------------------------
        setup_preload_fonts(content, ctx);
        return;
    }

    if let Ok(font_data) = std::fs::read(&font_filename) {
        content.screen.font_filename = font_filename.clone();
        get_font_list(content);

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_owned(font_data).tweak(FontTweak {
                scale: content.screen.font_scale, // make it smaller or taller
                y_offset_factor: content.screen.font_y_offset_factor, // move it down(+) or up(-)
                y_offset: content.screen.font_y_offset,
                baseline_offset_factor: -0.0333, // makes the default fonts look more centered in buttons and such
            }),
        );

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());
        //            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("my_font".to_owned());
        //            .insert(0, "my_font".to_owned());

        ctx.set_fonts(fonts);
    } else {
        // --------------------------------------
        // Default Fonts
        // --------------------------------------
        setup_preload_fonts(content, ctx);
    }

    content.fontbook = Default::default();
}

// ----------------------------------------------------------------------------
// Screen Update : Central Panel
// ----------------------------------------------------------------------------

impl eframe::App for Content {
    // ------------------------------------------------------------------------

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    // ------------------------------------------------------------------------
    //
    // eframe UPDATE()
    //
    // ------------------------------------------------------------------------

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // --------------------------------------
        // Load Window State
        // --------------------------------------
        self.screen.minimized = ctx.input(|i| i.viewport().minimized.unwrap_or(false));
        self.screen.maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        self.screen.fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));

        // --------------------------------------
        // Set Window Size
        // --------------------------------------

        // Notice : The problem of not filling the screen completely when turning on/off the windows decorated.
        let mut screen_rect = egui::Rect::ZERO;
        if !self.screen.decorated_skip_onetime {
            screen_rect = ctx.screen_rect();
            self.screen.window_size =
                egui::vec2(screen_rect.width().round(), screen_rect.height().round());
        }

        if self.screen.decorated_skip_onetime {
            self.screen.decorated_skip_onetime = false;
        }

        if !self.screen.fullscreen && !self.screen.maximized {
            let window_size = self.screen.window_size;
            if window_size != egui::vec2(screen_rect.width().round(), screen_rect.height().round())
            {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(window_size));
            }
        }

        let mut panel_frame = egui::Frame {
            inner_margin: Margin::symmetric(8.0, 8.0),
            outer_margin: 0.5.into(),
            rounding: 3.0.into(),
            shadow: epaint::Shadow::NONE,
            fill: ctx.style().visuals.window_fill(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            ..Default::default()
        };

        if is_basicmode(self, BasicMode::ClearGlassPanel) {
            panel_frame.fill =
                Color32::from_rgba_premultiplied(0, 0, 0, self.screen.clearglass_value);
        }
        if is_basicmode(self, BasicMode::SunGlassPanel) {
            panel_frame.fill =
                Color32::from_rgba_premultiplied(0, 0, 0, self.screen.sunglass_value);
        }
        if is_basicmode(self, BasicMode::BlackGlassPanel) {
            panel_frame.fill = Color32::from_rgba_premultiplied(
                self.screen.blackglass_value,
                self.screen.blackglass_value,
                self.screen.blackglass_value,
                255,
            );
        }
        if is_basicmode(self, BasicMode::WhiteGlassPanel) {
            panel_frame.fill = Color32::from_rgba_premultiplied(
                self.screen.whiteglass_value,
                self.screen.whiteglass_value,
                self.screen.whiteglass_value,
                255,
            );
        }

        // --------------------------------------
        // Central Panel
        // --------------------------------------
        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                // Frame Speed Check Data
                self.frame_history
                    .on_new_frame(ui.input(|i| i.time), frame.info().cpu_usage);

                // ----------------------------------
                // Startup Onetime
                //
                // if dos text window, Hide dos text window
                // ----------------------------------
                if is_basicmode(self, BasicMode::StartupOnetime) {
                    r_desk_startup_onetime(self, ui);
                    basicmode_off(self, BasicMode::StartupOnetime);
                }

                // ----------------------------------
                // Pre Style
                // ----------------------------------
                pre_style_update(self, ui);

                // ----------------------------------
                // Interact with the title bar (drag to move window):
                // ----------------------------------
                let app_rect = ui.max_rect();
                let title_bar_height = 60.0;
                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + title_bar_height;
                    rect
                };
                let title_bar_response = ui.interact(
                    title_bar_rect,
                    Id::new("title_bar"),
                    Sense::click_and_drag(),
                );

                if title_bar_response.double_clicked() {
                    if !self.screen.maximized {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
                        self.screen.maximized = true;
                    } else {
                        ui.ctx()
                            .send_viewport_cmd(ViewportCommand::Maximized(false));
                        self.screen.maximized = false;
                    }
                }

                if title_bar_response.drag_started_by(PointerButton::Primary) {
                    ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                }

                // ----------------------------------
                // Menu ( if !NoMenu )
                // ----------------------------------
                let menu_process = !is_basicmode(self, BasicMode::NoMenu);

                if menu_process {
                    // ------------------------------
                    // Main Title Bar & Title Bars
                    // ------------------------------
                    r_main_menu_bar_ui(self, ui);
                    ui.separator();

                    if is_basicmode(self, BasicMode::WindowSizeBar) {
                        window_size_bar_ui(self, ui);
                        ui.separator();
                    }

                    if is_basicmode(self, BasicMode::RDesk) {
                        r_desk_menu_bar_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::REditor) {
                        r_editor_menu_bar_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::RDos) {
                        r_dos_menu_bar_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::RChat) {
                        r_chat_menu_bar_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::PaintPanelMenu) {
                        r_paint_menu_bar_ui(self, ui);
                    }

                    // ------------------------------
                    // Main Panel
                    // ------------------------------
                    if is_basicmode(self, BasicMode::RDesk) {
                        r_desk_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::REditor) {
                        r_editor_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::RDos) {
                        r_dos_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::RChat) {
                        r_chat_ui(self, ui);
                    } else if is_basicmode(self, BasicMode::PaintPanel) {
                        r_paint_ui(self, ui);
                    }
                } else {
                    // ------------------------------
                    // No Menu Mode
                    // ------------------------------
                    no_menu_bar_ui(self, ui);
                    if is_basicmode(self, BasicMode::PaintPanel) {
                        r_paint_ui(self, ui);
                    }
                }

                // ----------------------------------
                // drag & dropped files:
                // ----------------------------------
                ui.input(|i| {
                    if !i.raw.dropped_files.is_empty() && self.editor.text_changed == false {
                        self.selected_path.path = i.raw.dropped_files[0]
                            .path
                            .clone()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_errpass()
                            .to_string();
                        read_file_reditor(self, self.selected_path.path.clone());
                    }
                });

                // ----------------------------------
                // Message Window Process
                // ----------------------------------
                message_window_process(self, ui);

                // ----------------------------------
                // HotKey
                // ----------------------------------
                update_hotkey_process(self, ui);

                update_etc_process(self, ui);

                // ----------------------------------
                // Auto LogOut
                // ----------------------------------
                // let elapsed_time: u64 = self.crypto.savetime_login.elapsed().as_secs();
                // if elapsed_time >= self.crypto.login_lifetime_seconds as u64 {
                let remaining_second = r_how_remaining_time_to_logout_auto(self);
                if remaining_second == 0 {
                    r_logout_accept_id_password(self, ui);
                    r_remaining_time_reset(self);

                    let message = "Auto LogOuted.";
                    show_set_message_window(self, "Notice", message, "", "", "");
                }

                //             ui_paint_test( ui.painter(), self);

                // --------------------------------------
                // Close
                // --------------------------------------

                // windows close event
                self.close_requested = ui.input(|i| i.viewport().close_requested());
                if self.close_requested {
                    request_close_rdesk(self, false);
                }

                // R-DESK close event
                if is_basicmode(self, BasicMode::CloseDesk) {
                    // if self.close_requested {
                    if self.screen.message_window.result_button == "Cancel"
                        || self.screen.message_window.result_button == "CLOSE"
                    {
                        basicmode_off(self, BasicMode::CloseDesk);
                        hide_message_window(self);
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    }

                    if self.screen.message_window.result_button == "Quit" {
                        hide_message_window(self);
                        self.close_cancelable = false;
                    }

                    // Close Decision
                    if is_basicmode(self, BasicMode::CloseDesk) && !self.close_cancelable {
                        r_logout_accept_id_password(self, ui);
                        // basicmode_off(self, BasicMode::CloseDesk);

                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                }

                /*
                if self.close_requested {
                    if self.close_cancelable {
                        // do nothing - we will close
                    } else {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    }

                    if !is_basicmode(self, BasicMode::CloseDesk) {
                    }
                }
                */
            }); // CentralPanel()

        // --------------------------------------
        // Using default after egui v0.24.0
        //
        // Zoom hotkey : Ctrl-'+', Ctrl-'-', Ctrl-'0'
        // On the web the browser controls the zoom
        // --------------------------------------
        if !frame.is_web() {
            // Using default after egui v0.24.0
            // egui::gui_zoom::zoom_with_keyboard_shortcuts(ui.ctx() frame.info().native_pixels_per_point);
            //            egui::gui_zoom::zoom_with_keyboard(ctx);
        }

        let mut is_request_repaint = false;

        if self.ad.is_ad_request_change {
            self.ad.is_ad_request_change = false;
            // is_request_repaint = true;
            // ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        if is_basicmode(self, BasicMode::FramePerSecond) {
            is_request_repaint = true;
            // is_request_repaint = false;
        }

        if is_request_repaint {
            // ctx.request_repaint();
            // 1000 millis / 60 fps = 16.67 millis
            // ctx.request_repaint_after(std::time::Duration::from_millis(8));
        }
    }
}

// ----------------------------------------------------------------------------
// R-DESK Startup Onetime
// ----------------------------------------------------------------------------

pub fn r_desk_startup_onetime(content: &mut Content, ui: &mut egui::Ui) {
    hide_doswindow_onetime(content);
    apply_main_features_setup(content, ui);

    // Apply Startup Image changes
    let image_startup_uri = get_text(content, "image-uri-startup:setup");
    if !image_startup_uri.trim().is_empty() && image_startup_uri != content.rdesk.command {
        content.rdesk.command = image_startup_uri.clone();
        content.rdesk.change_command = true;
    } else if content.rdesk.command.trim().is_empty() {
        content.rdesk.command.clear();
        content.rimage.image_onetime_internet = true;
        content.rimage.image_onetime_file = true;
        content.rimage.image_onetime_random = true;
    }

    pre_style_onetime(content, ui);
}

// ----------------------------------------------------------------------------

pub fn pre_style_onetime(content: &mut Content, ui: &mut egui::Ui) {
    let hovered_color = color32_dark_light(
        content,
        Color32::from_rgb(160, 190, 230),
        Color32::from_rgb(0, 60, 100),
    );
    let active_color = color32_dark_light(
        content,
        Color32::from_rgb(180, 210, 230),
        Color32::from_rgb(0, 100, 255),
    );

    ui.spacing_mut().indent = 8.0;

    ui.spacing_mut().scroll.floating_width = 3.0;
    ui.spacing_mut().scroll.bar_width = 13.0;
    ui.spacing_mut().scroll.bar_inner_margin = 0.0;

    ui.spacing_mut().slider_rail_height = 3.0;
    // ui.style_mut().interaction.show_tooltips_only_when_still = false;
    // ui.style_mut().interaction.tooltip_delay = 0.0;

    // let mut visuals = ui.visuals_mut().clone();
    // For egui::Window Title
    // visuals.window_highlight_topmost = false;

    // For egui::ScrollArea

    /*
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.widgets.hovered.fg_stroke.color = hovered_color;
    visuals.widgets.active.fg_stroke.color = active_color;
    visuals.text_cursor.on_duration = 0.7;
    visuals.text_cursor.off_duration = 0.3;
    ui.ctx().set_visuals(visuals);
    */

    /*
    ui.ctx().style_mut(|style| {
        style.visuals.widgets.hovered.fg_stroke.color = hovered_color;
        style.visuals.widgets.active.fg_stroke.color = active_color;
        style.visuals.text_cursor.on_duration = 0.7;
        style.visuals.text_cursor.off_duration = 0.3;
    });
    */

    // ui.ctx().set_visuals(visuals);

    /*
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.widgets.hovered.fg_stroke.color = hovered_color;
    visuals.widgets.active.fg_stroke.color = active_color;
    visuals.text_cursor.on_duration = 0.7;
    visuals.text_cursor.off_duration = 0.3;
    ui.ctx().set_visuals(visuals);
    */

    /*
    let mut visuals = ui.visuals().clone();
    visuals.widgets.hovered.fg_stroke.color = hovered_color;
    visuals.widgets.active.fg_stroke.color = active_color;
    visuals.text_cursor.on_duration = 0.7;
    visuals.text_cursor.off_duration = 0.3;
    ui.ctx().set_visuals(visuals);
    */

    // For egui::ScrollArea

    ui.visuals_mut().widgets.hovered.fg_stroke.color = hovered_color;
    ui.visuals_mut().widgets.active.fg_stroke.color = active_color;

    ui.visuals_mut().text_cursor.on_duration = 0.7;
    ui.visuals_mut().text_cursor.off_duration = 0.3;

    ui.style_mut().interaction.tooltip_delay = 0.3;
    ui.style_mut().interaction.tooltip_grace_time = 0.1;

    let mut style = ui.style_mut().clone();
    for (text_style, font_id) in style.text_styles.iter_mut() {
        if *text_style == TextStyle::Body {
            *font_id = FontId::monospace(content.screen.font_size_normal);
        }
        if *text_style == TextStyle::Button {
            *font_id = FontId::monospace(content.screen.font_size_normal);
            style.override_text_style = Some(TextStyle::Button);
        }
    }
    ui.ctx().set_style(style);

    // For fixed bug changed with set_style
    set_dark_light_mode(content, ui);
}

// ----------------------------------------------------------------------------

pub fn pre_style_update(content: &mut Content, ui: &mut egui::Ui) {
    // ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
    // color32_dark_light(content, Color32::from_rgb(50, 50, 50), Color32::from_rgb(230, 230, 230) );

    // ui.spacing_mut().indent = 8.0;

    // content.screen.dark_mode = ui.style().visuals.dark_mode;

    ui.spacing_mut().button_padding.x = content.screen.button_padding_x;
    ui.spacing_mut().button_padding.y = content.screen.button_padding_y;
    ui.spacing_mut().item_spacing.x = content.screen.item_spacing_x;
    ui.spacing_mut().item_spacing.y = content.screen.item_spacing_y;

    let hovered_color = color32_dark_light(
        content,
        Color32::from_rgb(160, 190, 230),
        Color32::from_rgb(0, 60, 100),
    );
    let active_color = color32_dark_light(
        content,
        Color32::from_rgb(180, 210, 230),
        Color32::from_rgb(0, 100, 255),
    );

    ui.visuals_mut().widgets.hovered.fg_stroke.color = hovered_color;
    ui.visuals_mut().widgets.active.fg_stroke.color = active_color;

    ui.visuals_mut().text_cursor.on_duration = 0.7;
    ui.visuals_mut().text_cursor.off_duration = 0.3;

    /*
    ui.visuals_mut().text_cursor.on_duration = 0.7;
    ui.visuals_mut().text_cursor.off_duration = 0.3;

    let mut text_cursor_style = TextCursorStyle::default();
    text_cursor_style.on_duration = 0.7;
    text_cursor_style.off_duration = 0.3;
    ui.visuals_mut().text_cursor = text_cursor_style;
    */

    // ui.spacing_mut().slider_rail_height = 3.0;
    // ui.style_mut().interaction.show_tooltips_only_when_still = false;
    // ui.style_mut().interaction.tooltip_delay = 0.0;

    /*
    let mut style = ui.style_mut().clone();
    for (text_style, font_id) in style.text_styles.iter_mut() {
        if *text_style == TextStyle::Body {
            *font_id = FontId::monospace(content.screen.font_size_normal);
        }
        if *text_style == TextStyle::Button {
            *font_id = FontId::monospace(content.screen.font_size_normal);
            style.override_text_style = Some(TextStyle::Button);
        }
    }
    ui.ctx().set_style(style);
    // For fixed bug changed with set_style
    set_dark_light_mode(content, ui);
    */

    /*
    let mut visuals = ui.visuals_mut().clone();
    // For egui::Window Title
    // visuals.window_highlight_topmost = false;

    // For egui::ScrollArea
    visuals.widgets.hovered.fg_stroke.color = hovered_color;
    visuals.widgets.active.fg_stroke.color = active_color;
    ui.ctx().set_visuals(visuals);
    */

    // pre_style_onetime(content, ui);
}

// ----------------------------------------------------------------------------
// Main HotKey Process
// ----------------------------------------------------------------------------

pub fn update_hotkey_process(content: &mut Content, ui: &mut egui::Ui) {
    if ui.input(|i| i.key_pressed(Key::Escape)) {
        mainmenu_toggle(content, BasicMode::REditor); // mainmenu clear & on
                                                      //            apply_r_panel_setup(content, ui);                                  // set Panel
        glasspannel_toggle(content, ui, BasicMode::BlackGlassPanel);
    }

    if ui.input(|i| i.key_pressed(Key::F1)) {
        //            glasspannel_toggle(content, ui, BasicMode::ClearGlassPanel);
        mainmenu_toggle(content, BasicMode::RDesk);
    }
    if ui.input(|i| i.key_pressed(Key::F2)) {
        //            glasspannel_toggle(content, ui, BasicMode::SunGlassPanel);
        mainmenu_toggle(content, BasicMode::REditor);
    }
    if ui.input(|i| i.key_pressed(Key::F3)) {
        //            glasspannel_toggle(content, ui, BasicMode::BlackGlassPanel);
        mainmenu_toggle(content, BasicMode::RDos);
    }
    if ui.input(|i| i.key_pressed(Key::F4)) {
        //            glasspannel_toggle(content, ui, BasicMode::WhiteGlassPanel);
        mainmenu_toggle(content, BasicMode::RChat);
    }
    if ui.input(|i| i.key_pressed(Key::F5)) {
        paintpanel_toggle(content, BasicMode::PaintPanel);
    }
    if ui.input(|i| i.key_pressed(Key::F6)) {
        glasspannel_toggle_next(content, ui);
        /*
                basicmode_toggle(content, BasicMode::RDeskImageSetup );
                basicmode_toggle(content, BasicMode::RProgramSetup );
                basicmode_toggle(content, BasicMode::RInternetSetup );
                submode_toggle(content, SubMode::RDosCommandSetup );
                basicmode_toggle(content, BasicMode::RChatSetup );
                basicmode_toggle(content, BasicMode::REditorSetup );
        */
    }

    // ----------------------------------
    // Data Information
    // ----------------------------------
    if ui.input(|i| i.key_pressed(Key::F7)) {
        dos_window_on_off(content, true);
        basicmode_toggle(content, BasicMode::DataInformation);
    }
    if is_basicmode(content, BasicMode::DataInformation) {
        data_information_ui(content, ui);
    }

    if ui.input(|i| i.key_pressed(Key::F8)) {
        dos_window_on_off(content, false);
        basicmode_off(content, BasicMode::DataInformation);
        ui.ctx().memory_mut(|mem| mem.reset_areas());
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    let window_title_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Keyword as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Keyword as usize],
    );
    let temp_color = window_title_color;

    if ui.input(|i| i.key_pressed(Key::F9)) {
        submode_toggle(content, SubMode::InspectionDebug);
    }
    if is_submode(content, SubMode::InspectionDebug) {
        let mut is_open_window: bool = true;
        egui::Window::new(RichText::new("🔍 Inspection").color(temp_color))
            .open(&mut is_open_window)
            .default_pos([120.0, 120.0])
            .default_size([800.0, 500.0])
            .max_size([1000.0, 600.0])
            .hscroll(true)
            .vscroll(true)
            .show(ui.ctx(), |ui| {
                let fps_text = format!("FPS : {}", FrameHistory::fps(&content.frame_history));
                ui.label(fps_text);

                ui.separator();

                let mut monitor_size: egui::Vec2 = Default::default();
                let mut outer_rect: egui::Rect = egui::Rect::ZERO;
                let mut inner_rect: egui::Rect = egui::Rect::ZERO;
                ui.ctx().input(|i| {
                    outer_rect = i.clone().viewport().outer_rect.unwrap_or(egui::Rect::ZERO);
                    inner_rect = i.clone().viewport().inner_rect.unwrap_or(egui::Rect::ZERO);
                    monitor_size = i.viewport().monitor_size.unwrap_or_default();
                });

                let text = format!("monitor_size : {}, {}", monitor_size.x, monitor_size.y);
                // ui.label_rich(text, FontId::monospace(15.0), Color32::LIGHT_GREEN);
                ui.label(text);

                let text = format!(
                    "outer_rect pos : {}, {}",
                    outer_rect.left(),
                    outer_rect.top()
                );
                ui.label(text);
                let text = format!(
                    "outer_rect end : {}, {}",
                    outer_rect.right(),
                    outer_rect.bottom()
                );
                ui.label(text);
                let text = format!(
                    "outer_rect size : {}, {}",
                    outer_rect.width(),
                    outer_rect.height()
                );
                ui.label(text);

                let text = format!(
                    "inner_rect pos : {}, {}",
                    inner_rect.left(),
                    inner_rect.top()
                );
                ui.label(text);
                let text = format!(
                    "inner_rect end : {}, {}",
                    inner_rect.right(),
                    inner_rect.bottom()
                );
                ui.label(text);
                let text = format!(
                    "inner_rect size : {}, {}",
                    inner_rect.width(),
                    inner_rect.height()
                );
                ui.label(text);

                let clip_rect = ui.clip_rect();
                let text = format!("clip_rect pos : {}, {}", clip_rect.left(), clip_rect.top());
                ui.label(text);
                let text = format!(
                    "clip_rect end : {}, {}",
                    clip_rect.right(),
                    clip_rect.bottom()
                );
                ui.label(text);
                let text = format!(
                    "clip_rect size : {}, {}",
                    clip_rect.width(),
                    clip_rect.height()
                );
                ui.label(text);

                let is_focused = ui.input(|i| i.focused);
                let text = format!(
                    "fullscreen: {}, maxmized: {}, minimized: {}, focused: {}",
                    content.screen.fullscreen,
                    content.screen.maximized,
                    content.screen.minimized,
                    is_focused
                );
                ui.label(text);
                /*
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
                let is_minimized = ui.input(|i| i.viewport().minimized.unwrap_or(false));
                let text = format!("is_maxmized(viewport): {}, is_minimized(viewport): {}", is_maximized, is_minimized );
                ui.label( text );
                */

                ui.separator();

                let tex_mngr = ui.ctx().tex_manager();
                let tex_mngr_read = tex_mngr.read();
                let textures: Vec<_> = tex_mngr_read.allocated().collect();
                for (&texture_id, meta) in textures {
                    ui.horizontal(|ui| {
                        let [w, h] = meta.size;
                        let size = egui::vec2(30.0, 20.0);
                        ui.image(egui::load::SizedTexture::new(texture_id, size))
                            .on_hover_ui(|ui| {
                                // show larger on hover
                                let max_size = 0.5 * ui.ctx().screen_rect().size();
                                let mut size = egui::vec2(w as f32, h as f32);
                                size *= max_size.x / size.x.max(max_size.x);
                                size *= max_size.y / size.y.max(max_size.y);
                                ui.image(egui::load::SizedTexture::new(texture_id, size));
                            });

                        ui.label(format!("{:?}", texture_id));
                        ui.label(format!("{:?}", meta.name));
                    });
                }

                ui.separator();

                ui.ctx().clone().inspection_ui(ui);
            });
        if is_open_window == false {
            submode_off(content, SubMode::InspectionDebug);
        }
    }

    if ui.input(|i| i.key_pressed(Key::F10)) {
        submode_toggle(content, SubMode::SettingDebug);
    }
    if is_submode(content, SubMode::SettingDebug) {
        let mut is_open_window: bool = true;
        egui::Window::new(RichText::new("🔧 Settings ").color(temp_color))
            .open(&mut is_open_window)
            .default_pos([140.0, 140.0])
            .auto_shrink(false)
            .vscroll(true)
            .show(ui.ctx(), |ui| {
                let fps = format!("FPS : {}", FrameHistory::fps(&content.frame_history));
                ui.label(fps);
                ui.separator();

                ui.ctx().clone().settings_ui(ui);
            });
        if is_open_window == false {
            submode_off(content, SubMode::SettingDebug);
        }
    }

    if ui.input(|i| i.key_pressed(Key::F11)) {
        submode_toggle(content, SubMode::MemoryDebug);
    }

    if is_submode(content, SubMode::MemoryDebug) {
        let mut is_open_window: bool = true;
        egui::Window::new(RichText::new("📝 Memory").color(temp_color))
            .open(&mut is_open_window)
            .resizable(false)
            .default_pos([100.0, 100.0])
            .show(ui.ctx(), |ui| {
                let fps = format!("FPS : {}", FrameHistory::fps(&content.frame_history));
                ui.label(fps);
                ui.separator();

                ui.ctx().clone().memory_ui(ui);
            });
        if is_open_window == false {
            submode_off(content, SubMode::MemoryDebug);
        }
    }

    // ------------------------------------------
    // F12 : Dos Screen
    // ------------------------------------------
    if ui.input(|i| i.key_pressed(Key::F12)) {
        let request_status = content.status_doswin_hide;
        dos_window_on_off(content, request_status);
    }

    // ------------------------------------------
    // Ctrl + F12 : Close Fast
    // ------------------------------------------
    if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, egui::Key::F12)) {
        request_close_rdesk(content, true);
    }

    // ------------------------------------------
    // CTRL + F1 : Master Etc : For DEV
    // ------------------------------------------
    if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, egui::Key::F1)) {
        basicmode_toggle(content, BasicMode::RMaster);
        //        submode_toggle(content, SubMode::RMasterEtc);
        submode_toggle(content, SubMode::RDevSpeech);
        //        submode_toggle(content, SubMode::RDevImage);
        submode_toggle(content, SubMode::RDevGemini);
    }
}

// ----------------------------------------------------------------------------
//  Update Etc Process
// ----------------------------------------------------------------------------

pub fn update_etc_process(content: &mut Content, ui: &mut egui::Ui) {
    // ------------------------------------------
    // R-CHAT StartUp
    // ------------------------------------------
    process_chat_startup(content);

    // ------------------------------------------
    // Paste Request
    // ------------------------------------------
    if is_basicmode(content, BasicMode::PasteRequest) {
        /*
                let ret = FileTree::paste_file_or_dir_cryptoable(content, ui);
                if ret == true {
                    basicmode_off(content, BasicMode::PasteRequest);
                    basicmode_off(content, BasicMode::OverWriteRequest);
                    basicmode_off(content, BasicMode::NumberedRequest);
                }
        */

        if !content.path_copymove.path.is_empty() {
            let ret = FileTree::paste_file_or_dir_cryptoable(content, ui);
            if ret == true {
                basicmode_off(content, BasicMode::PasteRequest);
                basicmode_off(content, BasicMode::OverWriteRequest);
                basicmode_off(content, BasicMode::NumberedRequest);
            }
        }
        if !content.encrypt_copymove.path.is_empty() {
            let ret = FileTree::paste_file_or_dir_encrypt(content, ui);
            if ret == true {
                basicmode_off(content, BasicMode::PasteRequest);
                basicmode_off(content, BasicMode::OverWriteRequest);
                basicmode_off(content, BasicMode::NumberedRequest);
            }
        }
        if !content.decrypt_copymove.path.is_empty() {
            let ret = FileTree::paste_file_or_dir_decrypt(content, ui);
            if ret == true {
                basicmode_off(content, BasicMode::PasteRequest);
                basicmode_off(content, BasicMode::OverWriteRequest);
                basicmode_off(content, BasicMode::NumberedRequest);
            }
        }
    }

    //    if is_submode(content, SubMode::RSpeechPlay) {
    //        text_to_speech_ready(content);
    //    }
}

// ----------------------------------------------------------------------------
//  Message Window Process
// ----------------------------------------------------------------------------

pub fn message_window_process(content: &mut Content, ui: &mut egui::Ui) {
    if !is_basicmode(content, BasicMode::MessageWindow) {
        return;
    }

    let result = show_message_window_process(content, ui);

    if is_submode(content, SubMode::REditorFileOpenApply) {
        if result == "Open" {
            submode_off(content, SubMode::REditorFileOpenApply);
            file_open_reditor(content, ui);
        } else if result == "Cancel" || result == "CLOSE" {
            submode_off(content, SubMode::REditorFileOpenApply);
        }

        content.screen.message_window.result_button.clear();
    }

    if result == "FONT RESET" {
        let key_text_font_filepath_default = "font-filepath:default";
        let font_filepath = get_text(content, key_text_font_filepath_default);
        content.screen.font_filename = font_filepath;

        let mut text = get_text(content, "font-scale:default");
        add_setup_str(content, "font-scale:setup", &text);
        text = get_text(content, "font-y-offset-factor:default");
        add_setup_str(content, "font-y-offset-factor:setup", &text);
        text = get_text(content, "font-y-offset:default");
        add_setup_str(content, "font-y-offset:setup", &text);

        setup_preload_fonts(content, ui.ctx());
        let key_text_font_filepath = "font-filepath:setup";
        add_setup_str(
            content,
            key_text_font_filepath,
            &content.screen.font_filename.clone(),
        );

        content.screen.message_window.result_button.clear();
    }

    if is_basicmode(content, BasicMode::PasteRequest) {
        if result == "OverWrite" {
            basicmode_on(content, BasicMode::OverWriteRequest);
        } else if result == "Numbered" {
            basicmode_on(content, BasicMode::NumberedRequest);
        } else if result == "Cancel" || result == "CLOSE" {
            basicmode_off(content, BasicMode::PasteRequest);
        }

        content.screen.message_window.result_button.clear();
    }

    if is_basicmode(content, BasicMode::DeletePathRequest) {
        if result == "DELETE" {
            let delete_path = content.selected_path.path.clone();
            let delete_result;

            let mut message: String;
            if content.selected_path.is_dir {
                let success = delete_file_with_trash(&delete_path.clone());
                message = get_text(content, "notice:delete-directory");
                message.push_str("\n");
                if success == true {
                    message.push_str(&get_text(content, "notice:moved-trash-dir"));
                    delete_result = Ok(());
                } else {
                    message.push_str(&get_text(content, "notice:without-trash-dir"));
                    delete_result = std::fs::remove_dir_all(delete_path.clone());
                }
            } else {
                let success = delete_file_with_trash(&delete_path.clone());
                message = get_text(content, "notice:delete-file");
                message.push_str("\n");
                if success == true {
                    message.push_str(&get_text(content, "notice:moved-trash-file"));
                    delete_result = Ok(());
                } else {
                    message.push_str(&get_text(content, "notice:without-trash-file"));
                    delete_result = std::fs::remove_file(delete_path.clone());
                }
            }

            let delete_path_text = match content.selected_path.is_dir {
                true => "Delete Folder",
                false => "Delete File",
            };

            let mut is_error_bg_color = false;
            message.push_str("\n\n");
            message.push_str(
                format!(
                    "< {} > {}",
                    delete_path_text,
                    make_logined_decrypt_path(content, delete_path.clone())
                )
                .as_str(),
            );
            if let Err(err) = delete_result {
                message.push_str("\n\n");
                let result_message = format!("< 🗙 Error Result > {}", err.to_string());
                message.push_str(&result_message);
                is_error_bg_color = true;
            } else {
                message.push_str("\n\n");
                let result_message = format!("[ Result : {} ]", "It's Deleted.");
                message.push_str(&result_message);
            }

            set_message_window(content, "Notice", &message, "", "OK", "");
            if is_error_bg_color {
                set_message_window_bg_color(content, TokenType::Special1, TokenType::Special2);
            }
            file_view_immediately(content);
            basicmode_off(content, BasicMode::DeletePathRequest);
            show_message_window(content);
        } else if result == "Cancel" || result == "CLOSE" {
            content.screen.message_window.result_button.clear();
            basicmode_off(content, BasicMode::DeletePathRequest);
        }

        content.screen.message_window.result_button.clear();
    }

    if is_basicmode(content, BasicMode::DataInformation) {
        if result == "Close" || result == "CLOSE" {
            basicmode_off(content, BasicMode::DataInformation);
            dos_window_on_off(content, false);
            let mut message_arc = match content.screen.message_window.message_arc.lock() {
                Ok(message) => message,
                Err(_err) => {
                    return;
                }
            };
            message_arc.clear();
        }

        content.screen.message_window.result_button.clear();
    }
}

// ----------------------------------------------------------------------------
// Request Close RDESK
// ----------------------------------------------------------------------------

pub fn request_close_rdesk(content: &mut Content, is_fast: bool) {
    if !is_fast {
        set_message_window(
            content,
            "⎆ Quit",
            "Do you want to Quit?",
            "Quit",
            "",
            "Cancel",
        );
        set_message_window_pos(content, 300.0, 100.0);
        show_message_window(content);
    } else {
        hide_message_window(content);
    }

    if content.editor.text_changed == true {
        let message = get_text(content, "warn:unsaved-doc");
        set_message_window(content, "Warning", &message, "Quit", "", "Cancel");
        set_message_window_bg_color(content, TokenType::Special1, TokenType::Special2);
        set_message_window_pos(content, 300.0, 100.0);
        show_message_window(content);
    } else if is_fast {
        content.close_cancelable = false;
    }

    basicmode_on(content, BasicMode::CloseDesk);
}

// ----------------------------------------------------------------------------

pub fn data_information_ui(content: &mut Content, ui: &mut egui::Ui) {
    /*
    let fps_text = format!("FPS : {}", FrameHistory::fps(&content.frame_history));
    println!("{}", fps_text);

    let mut monitor_size: egui::Vec2 = Default::default();
    let mut outer_rect: egui::Rect = egui::Rect::ZERO;
    let mut inner_rect: egui::Rect = egui::Rect::ZERO;
    ui.ctx().input(|i| {
        outer_rect = i.clone().viewport().outer_rect.unwrap_or(egui::Rect::ZERO);
        inner_rect = i.clone().viewport().inner_rect.unwrap_or(egui::Rect::ZERO);
        monitor_size = i.viewport().monitor_size.unwrap_or_default();
    });

    let text = format!("monitor_size : {}, {}", monitor_size.x, monitor_size.y);
    // ui.label_rich(text, FontId::monospace(15.0), Color32::LIGHT_GREEN);
    println!("{}", text);

    let text = format!(
        "outer_rect pos : {}, {}",
        outer_rect.left(),
        outer_rect.top()
    );
    println!("{}", text);
    let text = format!(
        "outer_rect end : {}, {}",
        outer_rect.right(),
        outer_rect.bottom()
    );
    println!("{}", text);
    let text = format!(
        "outer_rect size : {}, {}",
        outer_rect.width(),
        outer_rect.height()
    );
    println!("{}", text);

    let text = format!(
        "inner_rect pos : {}, {}",
        inner_rect.left(),
        inner_rect.top()
    );
    println!("{}", text);
    let text = format!(
        "inner_rect end : {}, {}",
        inner_rect.right(),
        inner_rect.bottom()
    );
    println!("{}", text);
    let text = format!(
        "inner_rect size : {}, {}",
        inner_rect.width(),
        inner_rect.height()
    );
    println!("{}", text);

    let clip_rect = ui.clip_rect();
    let text = format!("clip_rect pos : {}, {}", clip_rect.left(), clip_rect.top());
    println!("{}", text);
    let text = format!(
        "clip_rect end : {}, {}",
        clip_rect.right(),
        clip_rect.bottom()
    );
    println!("{}", text);
    let text = format!(
        "clip_rect size : {}, {}",
        clip_rect.width(),
        clip_rect.height()
    );
    println!("{}", text);

    let is_focused = ui.input(|i| i.focused);
    let text = format!(
        "fullscreen: {}, maxmized: {}, minimized: {}, focused: {}",
        content.screen.fullscreen, content.screen.maximized, content.screen.minimized, is_focused
    );
    println!("{}", text);

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    let is_minimized = ui.input(|i| i.viewport().minimized.unwrap_or(false));
    let text = format!(
        "is_maxmized(viewport): {}, is_minimized(viewport): {}",
        is_maximized, is_minimized
    );
    println!("{}", text);
    */

    // ------------------------------------------
    let mut message = String::new();

    let fps = format!(
        "Start FPS : {}\n",
        FrameHistory::fps(&content.frame_history).round()
    );
    message.push_str(&fps);

    let pixels_per_point = ui.ctx().pixels_per_point();
    let ppp = format!("pixels_per_point: {}\n", pixels_per_point);
    message.push_str(&ppp);

    let ui_available_height = ui.available_height();
    let ui_available_width = ui.available_width();
    let avail_w_h = format!(
        "ui_avail_width : {}, ui_avail_height : {}",
        ui_available_width, ui_available_height
    );
    message.push_str(&avail_w_h);

    show_set_message_window(content, "Information Test", &message, "", "Close", "");
}

// ----------------------------------------------------------------------------

pub fn hide_doswindow_onetime(content: &mut Content) {
    let name_cstring = std::ffi::CString::new(content.console_name.clone()).unwrap_errpass();
    unsafe { winapi::um::wincon::SetConsoleTitleA(name_cstring.clone().as_ptr()) }; // Rename Console Title

    for _i in 0..100 {
        let success = dos_window_on_off(content, false);
        if success {
            return;
        }
    }
}

// ----------------------------------------------------------------------------

pub fn get_text(content: &mut Content, key_text: &str) -> String {
    content.screen.lang_store.get_lang_text(key_text)
}

// ----------------------------------------------------------------------------

pub fn get_text_screen(screen: &mut ScreenData, key_text: &str) -> String {
    screen.lang_store.get_lang_text(key_text)
}

// ----------------------------------------------------------------------------

pub fn get_setup(content: &mut Content, key_text: &str) -> String {
    content.screen.lang_store.get_setup_text(key_text)
}

// ----------------------------------------------------------------------------

#[inline]
pub fn is_dark_mode(ui: &mut egui::Ui) -> bool {
    ui.style().visuals.dark_mode
}

#[inline]
pub fn is_light_mode(ui: &mut egui::Ui) -> bool {
    !(ui.style().visuals.dark_mode)
}

#[inline]
pub fn set_dark_mode(content: &mut Content, ui: &mut egui::Ui) {
    ui.painter().ctx().set_visuals(egui::Visuals::dark());
    // ui.visuals_mut().dark_mode = true;
    content.screen.dark_mode = true;
}

#[inline]
pub fn set_light_mode(content: &mut Content, ui: &mut egui::Ui) {
    ui.painter().ctx().set_visuals(egui::Visuals::light());
    // ui.visuals_mut().dark_mode = false;
    content.screen.dark_mode = false;
}

#[inline]
pub fn set_dark_light_mode(content: &mut Content, ui: &mut egui::Ui) {
    if content.screen.dark_mode {
        set_dark_mode(content, ui);
    } else {
        set_light_mode(content, ui);
    }
}

#[inline]
pub fn color32_dark_light(content: &mut Content, dark: Color32, light: Color32) -> Color32 {
    if content.screen.dark_mode {
        dark
    } else {
        light
    }
}

#[inline]
pub fn color32_dark_light_ui(ui: &mut egui::Ui, dark: Color32, light: Color32) -> Color32 {
    if is_dark_mode(ui) {
        dark
    } else {
        light
    }
}

#[inline]
pub fn color32_dark_light_screen(
    screen: &mut ScreenData,
    dark: Color32,
    light: Color32,
) -> Color32 {
    if screen.dark_mode {
        dark
    } else {
        light
    }
}

/*
#[inline]
pub fn color32_black_white(content: &mut Content, dark: Color32, light: Color32) -> Color32 {
    if is_basicmode(content, BasicMode::WhiteGlassPanel) { light }
    else { dark }
}
*/
// ----------------------------------------------------------------------------

pub trait Errpass<T> {
    fn unwrap_errpass(self) -> T;
}

impl<T: std::default::Default> Errpass<T> for Option<T> {
    #[inline]
    fn unwrap_errpass(self) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("called `unwrap_errpass()` on an `Err` value : None");
                std::default::Default::default()
            }
        }
    }
}

impl<T: std::default::Default, E> Errpass<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    #[inline]
    fn unwrap_errpass(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("called `unwrap_errpass()` on an `Err` value : {:?}", &e);
                std::default::Default::default()
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Glass Panel Menu Bar
// ----------------------------------------------------------------------------

fn glass_panel_menu_bar(content: &mut Content, ui: &mut egui::Ui) {
    let normal_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize]
            [TokenType::Punctuation as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize]
            [TokenType::Punctuation as usize],
    );
    let selected_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
    );
    let mut temp_color: Color32;

    //    let min_size: Vec2 = Vec2 { x: 0.0, y: content.font_size_normal * content.font_adjust + content.button_padding_y * 2.0 };
    let min_size: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    temp_color = normal_color;
    if is_basicmode(content, BasicMode::BlackGlassPanel) {
        temp_color = selected_color;
    }
    let temp_mark = if is_dark_mode(ui) {
        get_text(content, "r-blackglass:menu")
    } else {
        get_text(content, "r-whiteglass:menu")
    };
    //    if ui.button( RichText::new( temp_mark ).color( temp_color ) )
    let response = ui
        .add(egui::Button::new(RichText::new(temp_mark).color(temp_color)).min_size(min_size))
        .on_hover_text_at_pointer(get_text(content, "r-blackglass:help"));
    if response.clicked() {
        glasspannel_toggle(content, ui, BasicMode::BlackGlassPanel);
    }

    temp_color = normal_color;
    if is_basicmode(content, BasicMode::WhiteGlassPanel) {
        temp_color = selected_color;
    }
    let temp_mark = if is_light_mode(ui) {
        get_text(content, "r-blackglass:menu")
    } else {
        get_text(content, "r-whiteglass:menu")
    };
    //    if ui.button( RichText::new( temp_mark ).color( temp_color ) )
    let response = ui
        .add(egui::Button::new(RichText::new(temp_mark).color(temp_color)).min_size(min_size))
        .on_hover_text_at_pointer(get_text(content, "r-whiteglass:help"));
    if response.clicked() {
        glasspannel_toggle(content, ui, BasicMode::WhiteGlassPanel);
    }

    temp_color = normal_color;
    if is_basicmode(content, BasicMode::SunGlassPanel) {
        temp_color = selected_color;
    }
    //    if ui.button( RichText::new( get_text(content, "r-sunglass:menu") ).color( temp_color ) )
    let response = ui
        .add(
            egui::Button::new(
                RichText::new(get_text(content, "r-sunglass:menu")).color(temp_color),
            )
            .min_size(min_size),
        )
        .on_hover_text_at_pointer(get_text(content, "r-sunglass:help"));
    if response.clicked() {
        glasspannel_toggle(content, ui, BasicMode::SunGlassPanel);
    }

    temp_color = normal_color;
    if is_basicmode(content, BasicMode::ClearGlassPanel) {
        temp_color = selected_color;
    }
    //    if ui.button( RichText::new( get_text(content, "r-clearglass:menu") ).color( temp_color ) )
    let response = ui
        .add(
            egui::Button::new(
                RichText::new(get_text(content, "r-clearglass:menu")).color(temp_color),
            )
            .min_size(min_size),
        )
        .on_hover_text_at_pointer(get_text(content, "r-clearglass:help"));
    if response.clicked() {
        glasspannel_toggle(content, ui, BasicMode::ClearGlassPanel);
    }
}

// ----------------------------------------------------------------------------

pub fn apply_main_features_setup(content: &mut Content, ui: &mut egui::Ui) {
    let key_text = "alway-on-top:setup";
    let setup = get_text(content, key_text);
    if setup == "true" {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::AlwaysOnTop));
        content.screen.always_on_top = true;
    } else {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::Normal));
        content.screen.always_on_top = false;
    }

    let key_text = "r-desk-style:setup";
    let setup = get_text(content, key_text);
    if setup == "true" {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Decorations(true));
        content.screen.decorated = true;
        content.screen.decorated_skip_onetime = true;
    } else {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Decorations(false));
        content.screen.decorated = false;
        content.screen.decorated_skip_onetime = true;
    }

    let key_text = "dark-mode:setup";
    let setup = get_text(content, key_text);
    if setup == "true" {
        set_dark_mode(content, ui);
        // content.screen.dark_mode = true;
    } else {
        set_light_mode(content, ui);
        // content.screen.dark_mode = false;
    }

    let key_text = "window-sizebar:setup";
    let setup = get_text(content, key_text);
    if setup == "true" {
        basicmode_on(content, BasicMode::WindowSizeBar);
    } else {
        basicmode_off(content, BasicMode::WindowSizeBar);
    }

    //    basicmode_on(content, BasicMode::StartupOnetime);
}

// ----------------------------------------------------------------------------
// R-DESK UI
// ----------------------------------------------------------------------------

fn r_desk_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.screen.font_size_desk;
    let font_id = egui::FontId::monospace(font_size);
    let language = "console";
    preset_theme_color(content, ui, font_id.clone(), language);

    let rightpanel_width = content.screen.rightpanel_width;

    let ui_available_width = ui
        .available_width()
        .at_least(content.screen.window_size_min.x);

    let mut desired_width = ui_available_width;
    let spacing = (content.screen.item_spacing_x * 2.0) + 6.0;
    if is_submode(content, SubMode::RDeskLeftPanel) {
        desired_width -= content.screen.leftpanel_width + spacing;
    }
    if is_submode(content, SubMode::RDeskRightPanel) {
        desired_width -= rightpanel_width + spacing;
    }
    desired_width = desired_width.at_least(0.0);

    let ui_available_height = ui
        .available_height()
        .at_least(content.screen.window_size_min.y);
    let mut max_height = ui_available_height;

    if is_submode(content, SubMode::RDeskUriInput) {
        let row_height = ui.fonts(|f| f.row_height(&font_id.clone()));

        let mut max_height_inputline = (row_height * content.screen.font_adjust)
            + (content.screen.button_padding_y * 2.0)
            + (content.screen.item_spacing_y * 2.0)
            + 10.0;
        let single_min_height = 35.0;
        max_height_inputline = max_height_inputline.at_least(single_min_height);
        max_height = (max_height - max_height_inputline).at_least(0.0);
    }

    // ------------------------------------------
    // Audio To Text Ready
    // ------------------------------------------
    if is_submode(content, SubMode::RAudioToTextOpenai) {
        r_audio_to_text_openai_rdesk_ready(content);
        if !content.audio_text.ehttp.inprogress {
            submode_off(content, SubMode::RAudioToTextOpenai);
            // if is_submode(content, SubMode::RAudioToTextAuto) {
            //    submode_off(content, SubMode::RAudioToTextAuto);
            //    content.rdesk.change_command = true;
            // }
        }
    }

    ui.vertical(|ui| {
        ui.set_max_height(max_height);

        ui.horizontal_top(|ui| {
            // --------------------------------------
            // Left Panel File
            // --------------------------------------
            if is_submode(content, SubMode::RDeskLeftPanel) {
                ui.vertical(|ui| {
                    ui.set_max_width(content.screen.leftpanel_width);

                    r_ad_leftpanel_display(content, ui);

                    leftpanel_filetree_ui(content, ui);
                });

                // Resizeable ui.separator()
                separator_left_panel(&mut content.screen, ui);
            }

            ui.vertical(|ui| {
                ui.set_max_width(desired_width);

                ScrollArea::both()
                    .id_source("rdesk_image_view")
                    .auto_shrink([false, false])
                    .max_height(f32::INFINITY)
                    .max_width(desired_width)
                    .show(ui, |ui| {
                        let mut load_success: bool = false;

                        if is_basicmode(content, BasicMode::RDeskInternetImage) {
                            //                content.rimage.ehttp.start_trigger = true;
                            //                content.rimage.ehttp.streaming = true;
                            // Download the image from Internet
                            load_success = r_image_auto_download(content, ui);
                        } else if is_basicmode(content, BasicMode::RDeskFileImage) {
                            // Read the image from File
                            let save_onetime_random = content.rimage.image_onetime_random;
                            r_image_auto_fileload(content, ui);
                            if save_onetime_random != content.rimage.image_onetime_random {
                                load_success = true;
                            }
                        }

                        if content.rimage.ehttp.inprogress {
                            ui.add(egui::Spinner::new().size(50.0));

                            let inprogress_success = r_image_auto_download(content, ui);
                            if inprogress_success {
                                load_success = true;
                                rdesk_sub_toggle(content, BasicMode::RDeskInternetImage);
                            }
                        }

                        //            if content.rimage.image_scale_default == 0.0 {     // BUG FIX: content.rimage.image_scale_default becomes 0.0 after Window Minimize due to DragValue()
                        //                load_success = true;
                        //            }

                        // like Magnetic : If it is near 1.0, set it to 1.0
                        if content.rimage.image_scale > 0.91 && content.rimage.image_scale < 1.09 {
                            content.rimage.image_scale = 1.0;
                        }
                        // like Magnetic : If it is near content.rimage.image_scale_default, set it to content.rimage.image_scale_default
                        if content.rimage.image_scale > content.rimage.image_scale_default - 0.09
                            && content.rimage.image_scale
                                < content.rimage.image_scale_default + 0.09
                        {
                            content.rimage.image_scale = content.rimage.image_scale_default;
                        }

                        let max_size_mode = if content.rimage.image_scale == 1.0 {
                            true
                        } else {
                            false
                        };

                        if let Some(image) = &content.rimage.image_data {
                            let tlr = image.load_for_size(ui.ctx(), ui.available_size());
                            let original_image_size = tlr.as_ref().ok().and_then(|t| t.size());
                            let image_size = original_image_size.unwrap_or_default();

                            let scale_x = image_size[0] as f32 / desired_width;
                            let scale_y = image_size[1] as f32 / max_height;

                            content.rimage.image_scale_default = match scale_x < scale_y {
                                true => scale_x,
                                false => scale_y,
                            };
                            content.rimage.image_scale_default =
                                (content.rimage.image_scale_default * 100.0).round() / 100.0;
                            // 1.2345 => 1.23
                            // let message = format!("c.default : {}, image_size : {:?}, scale_x : {}, scale_y : {}", content.rimage.image_scale_default, image_size, scale_x, scale_y );
                            // show_set_message_window(content, "Scale", &message, "", "", "");
                        }

                        if max_size_mode || (load_success && content.rimage.image_scale != 1.0) {
                            content.rimage.image_scale = 1.0;
                        }

                        ui.with_layout(
                            egui::Layout::centered_and_justified(Direction::LeftToRight),
                            |ui| {
                                r_image_display(content, ui);
                            },
                        );
                    }); // ScrollArea::both()
            }); // ui.vertical()

            // ------------------------------------------
            // Right Panel Child View
            // ------------------------------------------
            if is_submode(content, SubMode::RDeskRightPanel) {
                // Resizeable ui.separator()
                separator_right_panel(&mut content.screen, ui);

                ui.vertical(|ui| {
                    r_ad_rightpanel_display(content, ui);

                    let _output = ScrollArea::both()
                        .id_source("rdesk_right_panel_view")
                        .auto_shrink([false, false])
                        .max_height(max_height)
                        .max_width(rightpanel_width)
                        //            .min_scrolled_width( rightpanel_childview_width )
                        .show(ui, |ui| {
                            rightpanel_cmd_child_view_ui(content, ui);
                        }); // ScrollArea::both() : right_panel_view
                }); // ui.vertical() : right_panel_view
            }
        }); // ui.horizontal_top()
    }); // ui.vertical()

    // ------------------------------------------
    // Uri Input_line
    // ------------------------------------------
    if is_submode(content, SubMode::RDeskUriInput) {
        ui.vertical(|ui| {
            ui.separator();

            ui.set_max_width(desired_width);

            r_desk_input_line_ui(content, ui);
        }); // ui.vertical() : input_line
    }

    // ------------------------------------------
    // R-DESK Command Process
    // ------------------------------------------
    r_desk_command_process(content, ui);

    // ------------------------------------------
    // create image download Ready & Get & Display
    // ------------------------------------------
    ready_request_openai(&mut content.o_image);

    get_create_image_opponent(content);

    display_o_image(content, ui);
    // ------------------------------------------

    if content.o_image.ehttp.inprogress {
        show_o_image_inprogress_window(content, ui);
    }
}

// ----------------------------------------------------------------------------

pub fn separator_left_panel(screen: &mut ScreenData, ui: &mut egui::Ui) {
    separator_left_right_panel(screen, ui, true);
}

// ----------------------------------------------------------------------------

pub fn separator_right_panel(screen: &mut ScreenData, ui: &mut egui::Ui) {
    separator_left_right_panel(screen, ui, false);
}

// ----------------------------------------------------------------------------

pub fn separator_left_right_panel(screen: &mut ScreenData, ui: &mut egui::Ui, is_left: bool) {
    let response = ui.separator();
    let drag_response = ui
        .allocate_rect(response.rect, Sense::click_and_drag())
        .on_hover_and_drag_cursor(CursorIcon::ResizeHorizontal)
        .on_hover_text_at_pointer("Double click or Right click to Default size.");

    if let Some(_pos) = drag_response.interact_pointer_pos() {
        let drag_delta = drag_response.drag_delta();
        if is_left {
            screen.leftpanel_width += drag_delta.x;
            screen.leftpanel_width = min_max_clamp_f32(
                screen.leftpanel_width,
                screen.leftpanel_width_min,
                screen.window_size.x / 3.0,
            );
        } else {
            screen.rightpanel_width += -(drag_delta.x);
            screen.rightpanel_width = min_max_clamp_f32(
                screen.rightpanel_width,
                screen.rightpanel_width_min,
                screen.window_size.x / 3.0,
            );
        }
    }

    if drag_response.double_clicked() || drag_response.secondary_clicked() {
        if is_left {
            screen.leftpanel_width = screen.leftpanel_width_default;
        } else {
            screen.rightpanel_width = screen.rightpanel_width_default;
        }
    }
}

// ----------------------------------------------------------------------------

pub fn r_desk_command_process(content: &mut Content, _ui: &mut egui::Ui) {
    if !content.rdesk.change_command {
        return;
    }
    content.rdesk.change_command = false;

    if content.rdesk.command.trim().is_empty() {
        return;
    }

    let input = content.rdesk.command.clone();
    content.rdesk.command.clear();
    add_command_history(&mut content.rdesk, input.clone());

    if input.starts_with("http:") || input.starts_with("https:") {
        content.rimage.image_onetime_random = false;
        content.rimage.image_onetime_internet = true;
        content.rimage.image_address = input.clone();
        rdesk_sub_toggle(content, BasicMode::RDeskInternetImage);
    } else if input.starts_with("drawed:") {
        let image = egui::Image::from_uri(input.clone());
        content.rimage.image_data = Some(image.clone());
        content.rdesk.command = image.uri().unwrap_or_default().to_string();
    } else if input.starts_with("draw") {
        // "draw" for voice or "draw:" for text
        content.o_image.image_request.prompt = input.clone();
        content
            .o_image
            .image_request
            .prompt
            .push_str(" 추가로 더 아름답고 더 멋있게 더 많은 사람들이 좋아하게 만들어줘.");
        run_o_image_request_openai(&mut content.openai, &mut content.o_image);

    //        let image = egui::Image::from_uri(input.clone());
    //        content.rimage.image_data = Some( image.clone() );
    //        content.rdesk.command = image.uri().unwrap_or_default().to_string();
    } else {
        content.rimage.image_onetime_random = false;
        content.rimage.image_onetime_file = true;
        content.rimage.image_filename = input.clone();
        rdesk_sub_toggle(content, BasicMode::RDeskFileImage);
    }
}

// ----------------------------------------------------------------------------
// R-DESK Uri Input line UI
// ----------------------------------------------------------------------------

pub fn r_desk_input_line_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.screen.font_size_desk;
    let font_id = egui::FontId::monospace(font_size);

    let normal_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::Console as usize]
            [TokenType::Punctuation as usize],
        content.screen.themecolor_light_vec[Theme::Console as usize]
            [TokenType::Punctuation as usize],
    );
    let selected_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::Console as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::Console as usize][TokenType::Literal as usize],
    );
    let selected_menu_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
    );
    let no_data_menu_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
    );
    let mut temp_color = normal_color;

    //    let min_size: Vec2 = Vec2 { x: 0.0, y: font_size * content.font_adjust + content.button_padding_y * 2.0 };
    let min_size: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    //    let row_height = font_size;

    let mut enter_width = content.screen.rightpanel_width - 100.0;
    enter_width = enter_width.at_least(font_size * 5.0);
    let button_list_size = (font_size + (content.screen.button_padding_x * 2.0)) * 2.0
        + (content.screen.item_spacing_x * 4.0)
        + (enter_width * 2.0)
        + 10.0;

    //    let mut enter_height = (font_size * content.font_adjust) + (content.button_padding_y * 2.0);
    let mut enter_height = min_size.y;
    let enter_min_height = 10.0;
    enter_height = enter_height.at_least(enter_min_height);

    // ------------------------------------------
    // R-DESK Uri input
    // ------------------------------------------
    ui.horizontal_top(|ui| {
//        ui.spacing_mut().button_padding.y = ( ( max_height_inputline - (row_height * content.font_adjust) - (content.item_spacing_y * 2.0) - 10.0 ) / 2.0 ).at_least(0.0);

        let mut addspace = 5.0;
        if is_submode(content, SubMode::RDeskLeftPanel) { addspace += content.screen.leftpanel_width + 10.0; }
        ui.add_space( addspace );

        temp_color = selected_color;
        let text = format!("＠ Uri ");
        ui.label( egui::RichText::new(&text).font(FontId::monospace( font_size )).color( temp_color ) )
        .on_hover_text_at_pointer( "draw: Draw Image Command | drawed: Drawed Image Display | https: or http: | File Path | CTRL+S : Save" );

        ui.visuals_mut().extreme_bg_color = egui::Color32::TRANSPARENT;

        let ui_available_width = ui.available_width().at_least(content.screen.window_size_min.x);
        // let is_fps = is_basicmode(content, BasicMode::FramePerSecond);

        let id_source = format!("{}_R_DESK_URI", content.crypto.bishop_idpassword.clone().unwrap_or_default());
        let command_line_desired_width = ( ui_available_width - button_list_size ).at_least(0.0);
        let command_line =
            ui.add( egui::TextEdit::singleline( &mut content.rdesk.command )
                            .id_source(id_source)
                            .desired_width( command_line_desired_width )
                            .cursor_at_end(true)
                            .lock_focus(true)
                            .return_key(None)
                            .font( font_id.clone() )
            );
        text_input_bodyguard(&mut content.rdesk.command, 1000);

        // --------------------------------------
        // Show history Text Button
        // --------------------------------------
        temp_color = selected_menu_color;
        if content.rdesk.command_history.is_empty() {
            temp_color = no_data_menu_color;
        }

        let menu_button_response =
        ui.menu_button(
        egui::RichText::new( get_text(content, "command-history:button") ).font(font_id.clone()).color( temp_color ),
        |ui| {
//            ui.spacing_mut().item_spacing.y = 0.0;
            let row_height = ui.fonts(|f| f.row_height( &font_id.clone() ));
            ui.set_max_height( (row_height + 5.0) * (content.rdesk.command_history.len() + 2) as f32 );

            temp_color = selected_color;
            ui.horizontal( |ui| {
                let title = format!("{:100}", get_text(content, "command-history-title:label") );
                let response = ui.add( egui::Button::new( RichText::new( title ).font( font_id.clone() ).color(temp_color) ).min_size(min_size) )
                .on_hover_text_at_pointer( get_text(content, "do-nothing:help") );
                if response.clicked() {
                    ui.close_menu();
                }
            });
            ui.separator();

            temp_color = normal_color;
            if !content.rdesk.command_history.is_empty() {
                let mut delete_request = false;
                let mut delete_index = 0;

                ScrollArea::both()
                .id_source("desk_command_history_scroll")
                .auto_shrink([false, false])
                .vscroll(true)
                .stick_to_bottom(true)
                .max_height( 600.0 )
//                .max_width( command_line_desired_width )
//                .min_scrolled_width( command_line_desired_width )
                .show(ui, |ui| {

                for i in 0..content.rdesk.command_history.len() {
                    ui.horizontal( |ui| {
                        temp_color = normal_color;
//                        if ui.button( RichText::new( get_text(content, "clear-input-clipboard:button") ).font(font_id.clone()).color(strong_menu_color) )
                        let response = ui.add( egui::Button::new( RichText::new( get_text(content, "clear-input-clipboard:button" ) ).font(font_id.clone()).color(temp_color) ).min_size(min_size) )
                        .on_hover_text_at_pointer( get_text(content, "clear-input-clipboard:help") );
                        if response.clicked()
                        {
                            ui.output_mut(|o| o.copied_text = content.rdesk.command_history[i].clone());
                            delete_request = true;
                            delete_index = i;
                        }

                        let command_history = format!("{:100}", str::replace(&content.rdesk.command_history[i].clone(), "\n", " "));
//                        if ui.button(RichText::new( command_history ).font( font_id.clone() ).color( temp_color ) )
                        let response =
                        ui.add( egui::Button::new( RichText::new( command_history ).font( font_id.clone() ).color(temp_color) ).min_size(min_size) )
                        .on_hover_text_at_pointer( format!("Num: {:3} / {}", i+1, content.rdesk.command_history.len()) );
                        if response.clicked() {
                            let save_command = content.rdesk.command.clone();
                            add_command_history(&mut content.rdesk, save_command);

                            content.rdesk.command = content.rdesk.command_history[i].clone();
                            if !content.rdesk.command.starts_with("draw:") {
                                content.rdesk.change_command = true;
                            }
                            content.rdesk.onetime_focus = true;
                            content.rdesk.end_of_cursor = true;
                            ui.close_menu();
                        }
                    });
                }
                });

                if delete_request {
                    delete_command_history(&mut content.rdesk, delete_index);
                }
            }
            else {
//                if ui.button(RichText::new( "None" ).font(font_id.clone()).color( temp_color ) )
                let response = ui.add( egui::Button::new( RichText::new( "None" ).font(font_id.clone()).color(temp_color) ).min_size(min_size) );
                if response.clicked() {
                    ui.close_menu();
                }
            }
        });

        menu_button_response.response
        .on_hover_text( get_text(content, "command-history:help") );

        if ui.input( |i| i.key_pressed(Key::ArrowUp) )
        || ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::ArrowUp))
        {
            content.rdesk.command = get_command_history_up(&mut content.rdesk);
            content.rdesk.onetime_focus = true;
            content.rdesk.end_of_cursor = true;
        }

        if ui.input( |i| i.key_pressed(Key::ArrowDown) )
        || ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::ArrowDown))
        {
            content.rdesk.command = get_command_history_down(&mut content.rdesk);
            content.rdesk.onetime_focus = true;
            content.rdesk.end_of_cursor = true;
        }

        // --------------------------------------
        // Copy Cut Clear Button
        // --------------------------------------
        let result = text_inputline_copy_cut_clear_button(&mut content.screen, ui, font_id.clone(), &mut content.rdesk.command, "cut");
        if result != "none" {
            content.rdesk.onetime_focus = true;
        }

        // --------------------------------------
        // Audio To Text Button
        // --------------------------------------
        let button_size = egui::vec2(enter_width, enter_height);
        inputline_record_button_audio2text(content, ui, button_size, font_size, false);

        // --------------------------------------
        // Enter Button
        // --------------------------------------
        temp_color = selected_menu_color;
        let enter_button =
        ui.add_sized([enter_width, enter_height],
            egui::Button::new( egui::RichText::new( get_text(content, "enter-input:button") ).font(font_id.clone()).color( temp_color ) )
        ).on_hover_text( get_text(content, "enter-input:help") );
        if enter_button.clicked() {
            content.rdesk.change_command = true;
        }

        if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, egui::Key::Enter))
        || ui.input(|i| i.key_released(egui::Key::Enter))                        // key_released()
        {
            if command_line.has_focus() {
                content.rdesk.change_command = true;
            } else {
                content.rdesk.onetime_focus = true;
            }
        }

        if content.rdesk.change_command {
//            content.rdesk.change_command = false;
            content.rdesk.onetime_focus = true;
        }

        if let Some(state) = egui::TextEdit::load_state(ui.ctx(), command_line.id) {
//            if let Some(ccursor_range) = state.ccursor_range() {
            if let Some(ccursor_range) = state.cursor.char_range() {
                content.rdesk_ccursor = ccursor_range.primary;
            }
        }

        // Cursor move : end of Line
        if content.rdesk.end_of_cursor {
            let text_edit_id = command_line.id;
            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                let ccursor = egui::text::CCursor::new( content.rdesk.command.chars().count() );
//                state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                state.cursor.set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                state.store(ui.ctx(), text_edit_id);
            }
            content.rdesk.end_of_cursor = false;
        }

        if content.rdesk.onetime_focus {
            content.rdesk.onetime_focus = false;
            command_line.request_focus();
        }

    }); // ui.horizoneal() : Command input
}

// ----------------------------------------------------------------------------
// R-DOS Normal View UI
// ----------------------------------------------------------------------------

pub fn r_dos_normal_view_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.dos.font_size;
    let font_id = egui::FontId::monospace(font_size);
    content.dos.text.clear();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0; // spacing adjustment

        for row in 0..content.console.get_buffer_height() {
            ui.horizontal(|ui| {
                let mut label_text = String::new();
                let mut save_attrib: u16 = 0x0007; // DOS default = 0x0007
                let mut job = egui::text::LayoutJob::default();
                let mut utf8_break = false;
                for col in 0..content.console.get_buffer_width() {
                    if utf8_break {
                        utf8_break = false;
                        continue;
                    }
                    let index = row * content.console.get_buffer_width() + col;
                    let label_c =
                        std::char::from_u32(content.console.get_buffer_char(index) as u32)
                            .unwrap_or(' ');
                    let width = unicode_width::UnicodeWidthChar::width_cjk(label_c).unwrap_or(2);
                    //                if label_c.len_utf8() >= 3 { utf8_break = true; }
                    if width >= 2 {
                        utf8_break = true;
                    }

                    let new_attrib: u16 = content.console.get_buffer_attrib(index);
                    if col == 0 {
                        save_attrib = new_attrib;
                    }

                    if new_attrib != save_attrib || col >= content.console.get_buffer_width() - 1 {
                        // color-by-color processing for speed

                        if col >= content.console.get_buffer_width() - 1 {
                            // Last a char
                            label_text.push(label_c);
                        }

                        let fgcolor: u16 = save_attrib & 0x000f;
                        let bgcolor: u16 = (save_attrib >> 4) & 0x000f;
                        let mut temp_fgcolor = DosColor::as_color32(fgcolor);
                        let mut temp_bgcolor = DosColor::as_color32(bgcolor);
                        if fgcolor == 7 {
                            temp_fgcolor = color32_dark_light(
                                content,
                                Color32::LIGHT_GRAY,
                                Color32::DARK_GRAY,
                            );
                        }
                        if bgcolor == 0 {
                            temp_bgcolor = Color32::TRANSPARENT;
                        } // background BLACK is TRANSPARENT

                        job.append(
                            &label_text,
                            0.0,
                            TextFormat {
                                color: temp_fgcolor,
                                background: temp_bgcolor,
                                font_id: font_id.clone(),
                                ..Default::default()
                            },
                        );

                        save_attrib = new_attrib;
                        label_text.clear();
                    }

                    if col < content.console.get_buffer_width() - 1 {
                        label_text.push(label_c);
                    }

                    content.dos.text.push(label_c);
                }
                content.dos.text.push('\n');
                ui.label(job);
                // ui.add_sized([0.0, 0.0], egui::Label::new(job))
            }); // ui.horizontal()
        }

        ui.label("\n");

        /*
                let message = content.console.get_string_console_attribute_x();
                let message = content.console.get_string_console_screen_x();
                show_set_message_window(content, "Notice", &message, "", "OK", "");
        */
    }); // ui.vertical()

    // To get TextEditOutput - Empty multiline()
    let mut empty_text = "";
    let text_edit = egui::TextEdit::multiline(&mut empty_text)
        .font(font_id.clone()) // for cursor height
        .desired_rows(0)
        .lock_focus(false);
    let mut output = text_edit.show(ui);

    let available_height = ui.available_height();
    r_dos_sub_ui(content, ui, available_height, &mut output);
}

// ----------------------------------------------------------------------------
// R-DOS Command Input UI
// ----------------------------------------------------------------------------

pub fn r_dos_input_line_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.dos.font_size;
    let font_id = egui::FontId::monospace(font_size);

    let normal_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::Console as usize]
            [TokenType::Punctuation as usize],
        content.screen.themecolor_light_vec[Theme::Console as usize]
            [TokenType::Punctuation as usize],
    );
    let selected_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::Console as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::Console as usize][TokenType::Literal as usize],
    );
    let selected_menu_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
    );
    let no_data_menu_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
    );
    let mut temp_color = normal_color;

    //    let min_size: Vec2 = Vec2 { x: 0.0, y: font_size * content.font_adjust + content.button_padding_y * 2.0 };
    let min_size: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    //    let row_height = font_size;

    let mut enter_width = content.screen.rightpanel_width - 100.0;
    enter_width = enter_width.at_least(font_size * 5.0);
    let button_list_size = (font_size + (content.screen.button_padding_x * 2.0)) * 2.0
        + (content.screen.item_spacing_x * 4.0)
        + (enter_width * 2.0)
        + 10.0;

    //    let mut enter_height = (font_size * content.font_adjust) + (content.button_padding_y * 2.0);
    let mut enter_height = min_size.y;
    let enter_min_height = 10.0;
    enter_height = enter_height.at_least(enter_min_height);

    // ------------------------------------------
    // DOS prompt & command input
    // ------------------------------------------
    ui.horizontal_top(|ui| {
        //        ui.spacing_mut().button_padding.y = ( ( max_height_inputline - (row_height * content.font_adjust) - (content.item_spacing_y * 2.0) - 10.0 ) / 2.0 ).at_least(0.0);

        let mut addspace = 5.0;
        if is_submode(content, SubMode::RDosLeftPanel) {
            addspace += content.screen.leftpanel_width + 10.0;
        }
        ui.add_space(addspace);

        temp_color = selected_color;
        if !content.rdos.is_prompt_on {
            temp_color = no_data_menu_color;
        }
        let current_dir = std::env::current_dir().unwrap_errpass();
        let prompt = format!("{}>", current_dir.to_str().unwrap_errpass());
        ui.label(
            egui::RichText::new(&prompt)
                .font(FontId::monospace(font_size))
                .color(temp_color),
        );

        ui.visuals_mut().extreme_bg_color = egui::Color32::TRANSPARENT;

        let ui_available_width = ui
            .available_width()
            .at_least(content.screen.window_size_min.x);
        // let is_fps = is_basicmode(content, BasicMode::FramePerSecond);

        let id_source = format!(
            "{}_R_DOS",
            content.crypto.bishop_idpassword.clone().unwrap_or_default()
        );
        let command_line_desired_width = (ui_available_width - button_list_size).at_least(0.0);
        let command_line = ui.add(
            egui::TextEdit::singleline(&mut content.rdos.command)
                .id_source(id_source)
                .desired_width(command_line_desired_width)
                .cursor_at_end(true)
                .lock_focus(true)
                .return_key(None)
                .font(font_id.clone()),
        );
        text_input_bodyguard(&mut content.rdos.command, 300);

        // --------------------------------------
        // Show history Text Button
        // --------------------------------------
        temp_color = selected_menu_color;
        if content.rdos.command_history.is_empty() {
            temp_color = no_data_menu_color;
        }

        let menu_button_response = ui.menu_button(
            egui::RichText::new(get_text(content, "command-history:button"))
                .font(font_id.clone())
                .color(temp_color),
            |ui| {
                //            ui.spacing_mut().item_spacing.y = 0.0;
                let row_height = ui.fonts(|f| f.row_height(&font_id.clone()));
                ui.set_max_height(
                    (row_height + 5.0) * (content.rdos.command_history.len() + 2) as f32,
                );

                temp_color = selected_color;
                ui.horizontal(|ui| {
                    let title = format!("{:100}", get_text(content, "command-history-title:label"));
                    let response = ui
                        .add(
                            egui::Button::new(
                                RichText::new(title).font(font_id.clone()).color(temp_color),
                            )
                            .min_size(min_size),
                        )
                        .on_hover_text_at_pointer(get_text(content, "do-nothing:help"));
                    if response.clicked() {
                        ui.close_menu();
                    }
                });
                ui.separator();

                temp_color = normal_color;
                if !content.rdos.command_history.is_empty() {
                    let mut delete_request = false;
                    let mut delete_index = 0;

                    ScrollArea::both()
                        .id_source("dos_command_history_scroll")
                        .auto_shrink([false, false])
                        .vscroll(true)
                        .stick_to_bottom(true)
                        .max_height(600.0)
                        //                .max_width( command_line_desired_width )
                        //                .min_scrolled_width( command_line_desired_width )
                        .show(ui, |ui| {
                            for i in 0..content.rdos.command_history.len() {
                                ui.horizontal(|ui| {
                                    temp_color = normal_color;
                                    //                        if ui.button( RichText::new( get_text(content, "clear-input-clipboard:button") ).font(font_id.clone()).color(strong_menu_color) )
                                    let response = ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new(get_text(
                                                    content,
                                                    "clear-input-clipboard:button",
                                                ))
                                                .font(font_id.clone())
                                                .color(temp_color),
                                            )
                                            .min_size(min_size),
                                        )
                                        .on_hover_text_at_pointer(get_text(
                                            content,
                                            "clear-input-clipboard:help",
                                        ));
                                    if response.clicked() {
                                        ui.output_mut(|o| {
                                            o.copied_text = content.rdos.command_history[i].clone()
                                        });
                                        delete_request = true;
                                        delete_index = i;
                                    }

                                    let command_history = format!(
                                        "{:100}",
                                        str::replace(
                                            &content.rdos.command_history[i].clone(),
                                            "\n",
                                            " "
                                        )
                                    );
                                    //                        if ui.button(RichText::new( command_history ).font( font_id.clone() ).color( temp_color ) )
                                    let response = ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new(command_history)
                                                    .font(font_id.clone())
                                                    .color(temp_color),
                                            )
                                            .min_size(min_size),
                                        )
                                        .on_hover_text_at_pointer(format!(
                                            "Num: {:3} / {}",
                                            i + 1,
                                            content.rdos.command_history.len()
                                        ));
                                    if response.clicked() {
                                        let save_command = content.rdos.command.clone();
                                        add_command_history(&mut content.rdos, save_command);
                                        content.rdos.command =
                                            content.rdos.command_history[i].clone();
                                        content.rdos.onetime_focus = true;
                                        content.rdos.end_of_cursor = true;
                                        ui.close_menu();
                                    }
                                });
                            }
                        });

                    if delete_request {
                        delete_command_history(&mut content.rdos, delete_index);
                    }
                } else {
                    //                if ui.button(RichText::new( "None" ).font(font_id.clone()).color( temp_color ) )
                    let response = ui.add(
                        egui::Button::new(
                            RichText::new("None")
                                .font(font_id.clone())
                                .color(temp_color),
                        )
                        .min_size(min_size),
                    );
                    if response.clicked() {
                        ui.close_menu();
                    }
                }
            },
        );

        menu_button_response
            .response
            .on_hover_text(get_text(content, "command-history:help"));

        if ui.input(|i| i.key_pressed(Key::ArrowUp))
            || ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::ArrowUp))
        {
            content.rdos.command = get_command_history_up(&mut content.rdos);
            content.rdos.onetime_focus = true;
            content.rdos.end_of_cursor = true;
        }

        if ui.input(|i| i.key_pressed(Key::ArrowDown))
            || ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::ArrowDown))
        {
            content.rdos.command = get_command_history_down(&mut content.rdos);
            content.rdos.onetime_focus = true;
            content.rdos.end_of_cursor = true;
        }

        // --------------------------------------
        // Copy Cut Clear Button
        // --------------------------------------
        let result = text_inputline_copy_cut_clear_button(
            &mut content.screen,
            ui,
            font_id.clone(),
            &mut content.rdos.command,
            "cut",
        );
        if result != "none" {
            content.rdos.onetime_focus = true;
        }

        // --------------------------------------
        // Audio To Text Button
        // --------------------------------------
        let button_size = egui::vec2(enter_width, enter_height);
        inputline_record_button_audio2text(content, ui, button_size, font_size, false);

        // --------------------------------------
        // Enter Button
        // --------------------------------------
        temp_color = selected_menu_color;
        let enter_button = ui
            .add_sized(
                [enter_width, enter_height],
                egui::Button::new(
                    egui::RichText::new(get_text(content, "enter-input:button"))
                        .font(font_id.clone())
                        .color(temp_color),
                ),
            )
            .on_hover_text(get_text(content, "enter-input:help"));
        if enter_button.clicked() {
            content.rdos.change_command = true;
        }

        /*
                if ui.input(|i| i.key_released(egui::Key::Tab))              // key_released()
                {
                    if content.rdos.command.trim() != "" {
                        let deletion_index: usize = content.rdos_ccursor.index;
                        let byte_index: usize = content.rdos.command.char_indices().nth(deletion_index).map_or_else(
                            || content.rdos.command.len(),
                            |(idx, _)| idx,
                        );
                        if content.rdos.command.chars().nth(deletion_index) == Some('\t') {
                            content.rdos.command.remove(byte_index);
                        }
                        content.rdos.change_command = true;
                    }
                }
        */

        if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, egui::Key::Enter))
            || ui.input(|i| i.key_released(egui::Key::Enter))
        // key_released()
        {
            if command_line.has_focus() {
                content.rdos.change_command = true;
            } else {
                content.rdos.onetime_focus = true;
            }
        }

        if content.rdos.change_command {
            //            content.rdos.change_command = false;
            content.rdos.onetime_focus = true;
        }

        if let Some(state) = egui::TextEdit::load_state(ui.ctx(), command_line.id) {
            //            if let Some(ccursor_range) = state.ccursor_range() {
            if let Some(ccursor_range) = state.cursor.char_range() {
                content.dos.ccursor = ccursor_range.primary;
            }
        }

        // Cursor move : end of Line
        if content.rdos.end_of_cursor {
            let text_edit_id = command_line.id;
            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                let ccursor = egui::text::CCursor::new(content.rdos.command.chars().count());
                //                state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                state
                    .cursor
                    .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                state.store(ui.ctx(), text_edit_id);
            }
            content.rdos.end_of_cursor = false;
        }

        if content.rdos.onetime_focus {
            content.rdos.onetime_focus = false;
            command_line.request_focus();
        }
    }); // ui.horizoneal() : Command input
}

// ----------------------------------------------------------------------------
// R-DOS UI
// ----------------------------------------------------------------------------

fn r_dos_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.dos.font_size;
    let font_id = egui::FontId::monospace(font_size);

    let rightpanel_width = content.screen.rightpanel_width;

    let ui_available_width = ui
        .available_width()
        .at_least(content.screen.window_size_min.x);

    let mut desired_width = ui_available_width;
    let spacing = (content.screen.item_spacing_x * 2.0) + 6.0;
    if is_submode(content, SubMode::RDosLeftPanel) {
        desired_width -= content.screen.leftpanel_width + spacing;
    }
    if is_submode(content, SubMode::RDosRightPanel) {
        desired_width -= rightpanel_width + spacing;
    }
    desired_width = desired_width.at_least(0.0);

    let ui_available_height = ui
        .available_height()
        .at_least(content.screen.window_size_min.y);
    let mut max_height = ui_available_height;
    let row_height = ui.fonts(|f| f.row_height(&font_id.clone()));

    let mut max_height_inputline = (row_height * content.screen.font_adjust)
        + (content.screen.button_padding_y * 2.0)
        + (content.screen.item_spacing_y * 2.0)
        + 10.0;
    let single_min_height = 35.0;
    max_height_inputline = max_height_inputline.at_least(single_min_height);
    max_height = (max_height - max_height_inputline).at_least(0.0);

    // ------------------------------------------
    // Audio To Text Ready
    // ------------------------------------------
    if is_submode(content, SubMode::RAudioToTextOpenai) {
        r_audio_to_text_openai_rdos_ready(content);
        if !content.audio_text.ehttp.inprogress {
            submode_off(content, SubMode::RAudioToTextOpenai);
            if is_submode(content, SubMode::RAudioToTextAuto) {
                // Currently not in use R-DOS
                submode_off(content, SubMode::RAudioToTextAuto);
                content.rdos.change_command = true;
            }
        }
    }

    // ------------------------------------------
    // DOS Console Screen Access Time for speed
    // ------------------------------------------
    let access_time = access_time(&mut content.savetime_dos, 0.1);
    if access_time {
        if is_basicmode(content, BasicMode::RDosBasic) {
            let string_console = content.console.get_string_console_screen();
            content.dos.text.clear();
            content.dos.text.push_str(&string_console);
        } else {
            // let string_console = content.console.get_string_console_screen();
            // content.dos.text.clear();
            // content.dos.text.push_str(&string_console);
            let _result = content.console.read_consolescreen();
            // if result == true {
            //     content.dos.text.clear();                     // Since it is done in access_time(), do not use it here.
            // }
        }
    }

    ui.vertical(|ui| {
        ui.set_max_height(max_height);

        ui.horizontal_top(|ui| {
            // --------------------------------------
            // Left Panel File
            // --------------------------------------
            if is_submode(content, SubMode::RDosLeftPanel) {
                ui.vertical(|ui| {
                    ui.set_max_width(content.screen.leftpanel_width);

                    r_ad_leftpanel_display(content, ui);
                    leftpanel_filetree_ui(content, ui);
                });

                // Resizeable ui.separator()
                separator_left_panel(&mut content.screen, ui);
            }

            ui.vertical(|ui| {
                ui.set_max_width(desired_width);

                ScrollArea::vertical()
                    .id_source("r_dos_scroll_vertical")
                    .auto_shrink([false, false])
                    .hscroll(false)
                    .vscroll(true)
                    //       .scroll2([true, true])
                    .max_height(max_height)
                    .max_width(desired_width)
                    .stick_to_right(false)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            ui.set_max_height(max_height);

                            // --------------------------------------
                            // Count Line Number
                            // --------------------------------------
                            if is_submode(content, SubMode::RDosLeftCounter)
                            //    && !is_basicmode(content, BasicMode::RDosNormal)
                            {
                                r_count_line_display(
                                    &mut content.screen,
                                    ui,
                                    &mut content.dos,
                                    font_id.clone(),
                                );
                            }

                            let text_view_width = (desired_width - 10.0).at_least(0.0);

                            ScrollArea::both()
                                .id_source("r_dos_scroll_horizontal")
                                .auto_shrink([false, false])
                                .hscroll(true)
                                .vscroll(false)
                                .max_height(max_height)
                                .max_width(text_view_width)
                                .stick_to_right(false)
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    // ----------------------------------
                                    // R-DOS Basic : DOS Console Screen
                                    // ----------------------------------
                                    if is_basicmode(content, BasicMode::RDosBasic) {
                                        r_dos_basic_view_ui(content, ui, max_height);
                                    }

                                    // ----------------------------------
                                    // R-DOS Normal : DOS Console Screen
                                    // ----------------------------------
                                    if is_basicmode(content, BasicMode::RDosNormal) {
                                        //                   r_dos_basic_view_ui(content, ui, max_height);
                                        r_dos_normal_view_ui(content, ui);
                                    }

                                    if content.dos.text_len != content.dos.text.len() {
                                        content.dos.text_len = content.dos.text.len();
                                        content.dos.text_changed = true;
                                    }
                                }); // ScrollArea::both(), hscroll(true) : DOS Screen
                        }); // ui.horizontal_top() : line_number, DOS Screen

                        if content.dos.text_changed
                            || content.rdos.change_prompt
                            || content.rdos.onetime_focus
                        {
                            //                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            ui.scroll_to_cursor(Some(Align::LEFT));
                            content.dos.text_changed = false;
                        }
                    }); // ScrollArea::vertical() : r_dos_view_vertical - count_line, DOS Screen
            }); // ui.vertical() : count_line, DOS Screen

            // --------------------------------------
            // Right Panel Child View
            // --------------------------------------
            if is_submode(content, SubMode::RDosRightPanel) {
                // Resizeable ui.separator()
                separator_right_panel(&mut content.screen, ui);

                ui.vertical(|ui| {
                    r_ad_rightpanel_display(content, ui);

                    let _output = ScrollArea::both()
                        .id_source("r_dos_right_panel_view")
                        .auto_shrink([false, false])
                        .max_height(max_height)
                        .max_width(rightpanel_width)
                        .show(ui, |ui| {
                            rightpanel_cmd_child_view_ui(content, ui);
                        });
                }); // ui.vertical() : right_panel_view
            }
        }); // ui.horizontal_top()

        // ------------------------------------------
        // DOS prompt & command input_line
        // ------------------------------------------
        ui.vertical(|ui| {
            ui.separator();

            ui.set_max_width(desired_width);

            r_dos_input_line_ui(content, ui);
        }); // ui.vertical() : input_line
    }); // ui.vertical() : ALL

    // ------------------------------------------
    // DOS command process
    // ------------------------------------------
    rustbasic_dos(content);
}

// ----------------------------------------------------------------------------

pub fn r_count_line_display(
    screen: &mut ScreenData,
    ui: &mut egui::Ui,
    textedit: &mut TextEditData,
    font_id: egui::FontId,
) {
    let font_size = textedit.font_size;
    // let font_id = egui::FontId::monospace(font_size);

    let strong_color = color32_dark_light_ui(
        ui,
        Color32::WHITE, // content.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Struct as usize],
        Color32::BLACK, // content.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Struct as usize]
    );
    let temp_color = color32_dark_light_ui(ui, Color32::DARK_GRAY, Color32::GRAY);
    let mut max_number = textedit.desired_rows; // max_number adjustment
    if max_number <= textedit.total_lines {
        max_number = textedit.total_lines;
    }

    let row_height = ui.fonts(|f| f.row_height(&font_id.clone()));

    // let message: String = String::new();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0; // spacing adjustment

        ui.label(
            // spacing adjustment
            egui::RichText::new("     ")
                .font(FontId::monospace(2.0))
                .color(temp_color),
        );

        let one_char_width = ui.fonts(|f| f.glyph_width(&font_id.clone(), ' '));
        for number in 1..=max_number {
            //            let mut row_size = textedit.row_height;
            let mut row_size = row_height;
            if number <= textedit.row_height_vec.len() {
                let temp_height = textedit.row_height_vec[number - 1];
                if temp_height != 0.0 {
                    row_size = temp_height;
                }

                let mut adjust: f32 = 0.0;
                if is_mode(screen.basic_mode, BasicMode::RDos)
                    && is_mode(screen.basic_mode, BasicMode::RDosNormal)
                {
                    adjust = 0.0;
                } else {
                    if number <= 3000 {
                        adjust = 0.0;
                    } else if number <= 7500 {
                        adjust = -(font_size * 0.0005);
                    } else if number <= 10000 {
                        adjust = font_size * 0.0015;
                    } else if number <= 12000 {
                        adjust = font_size * 0.0005;
                    } else if number <= 15000 {
                        // adjust = font_size * 0.001;
                        adjust = 0.0;
                    } else if number > 15000 {
                        adjust = font_size * 0.0045;
                    }
                }
                row_size = row_size + adjust;

                /*
                message = format!("{}: row_height: {}, temp_height: {}, row_size: {}", number, row_height, temp_height, row_size);
                if number == textedit.current_line {
                let message = format!("{}: row_height: {}, temp_height: {}, row_size: {}", number, row_height, temp_height, row_size);
                show_set_message_window(content, "height", &&message, "", "", "");
                }
                */
            }

            let mut num_color = temp_color;
            if number == textedit.current_line {
                num_color = strong_color;
            }
            let number_response: Response = if is_mode(screen.basic_mode, BasicMode::RDos)
                && is_mode(screen.basic_mode, BasicMode::RDosNormal)
            {
                ui.add_sized(
                    [one_char_width * 6.0, row_size],
                    egui::Label::new(
                        egui::RichText::new(format!("{:>5}", number))
                            // .font(FontId::monospace(font_size - 1.0))
                            .font(font_id.clone())
                            .color(num_color),
                    ),
                )
            } else {
                ui.add_sized(
                    [one_char_width * 6.0, row_size],
                    egui::Label::new(
                        egui::RichText::new(format!("{:>5}", number))
                            .font(FontId::monospace(font_size - 1.0))
                            .color(num_color),
                    ),
                )
            };

            let rect = number_response.rect;
            let radius = 0.2 * ui.spacing().interact_size.y;
            let rect_fill = Color32::TRANSPARENT;
            let bg_stroke = ui.style_mut().visuals.selection.stroke;

            if number_response.hovered() {
                ui.painter().rect(rect, radius, rect_fill, bg_stroke);
            }

            let number_response = number_response.interact(egui::Sense::click());
            if number_response.clicked() {
                let index = get_index_by_line_col(textedit, number, 1);
                textedit.move_cursor = Some(index);
                textedit.onetime_focus = true;
            }

            if number == textedit.current_line {
                ui.painter().rect(rect, radius, rect_fill, bg_stroke);
            }
        }
    });
}

// ----------------------------------------------------------------------------
// R-Editor UI
// ----------------------------------------------------------------------------

fn r_editor_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.editor.font_size;
    let font_id = egui::FontId::monospace(font_size);

    let normal_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize]
            [TokenType::Punctuation as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize]
            [TokenType::Punctuation as usize],
    );
    let temp_color = normal_color;

    let rightpanel_microview_width = content.screen.rightpanel_width;

    let ui_available_width = ui
        .available_width()
        .at_least(content.screen.window_size_min.x);

    let mut desired_width = ui_available_width;
    let spacing = (content.screen.item_spacing_x * 2.0) + 6.0;
    if is_submode(content, SubMode::REditorLeftPanel) {
        desired_width -= content.screen.leftpanel_width + spacing;
    }
    if is_submode(content, SubMode::REditorRightPanel) {
        desired_width -= rightpanel_microview_width + spacing;
    }
    desired_width = desired_width.at_least(0.0);

    let ui_available_height = ui
        .available_height()
        .at_least(content.screen.window_size_min.y);

    let max_height = (ui_available_height - 35.0).at_least(0.0); // ui_available_height - r_editor_status_bar_ui size
    let row_height = ui.fonts(|f| f.row_height(&font_id.clone()));
    //    let desired_rows: usize  = ( max_height / row_height ) as usize;
    content.editor.desired_rows = (max_height / row_height) as usize + 1;
    let desired_rows: usize = content.editor.desired_rows.clone();
    //    if content.editor.total_lines > desired_rows {
    //        row_height = content.editor.row_height;
    //    }

    // ------------------------------------------
    // Freedom Cursor
    // ------------------------------------------
    if is_basicmode(content, BasicMode::FreedomCursor) && desired_width > 0.0 {
        // minimize BUG (add space 2000 per a line) FIX : desired_width > 0.0
        let one_char_width = ui.fonts(|f| f.glyph_width(&font_id.clone(), ' '));
        let mut column: usize = (desired_width / one_char_width) as usize;
        if is_submode(content, SubMode::REditorLeftCounter) {
            column -= 6;
        }

        content.editor.freedom_row = desired_rows;
        content.editor.freedom_column = column;

        let access_time = access_time(&mut content.savetime_freecursor, 3.0); // for speed
        if !content.editor.freedom_request_ok || access_time {
            if access_time {
                clear_freedom_textdata_for_work(content);
            }
            add_freedom_textdata(content);
        }
    }

    ui.vertical(|ui| {
        ui.set_max_height(max_height);

        // ------------------------------------------
        // Multi Editor LEFT-CENTER-RIGHT
        // ------------------------------------------
        ui.horizontal_top(|ui| {
            // --------------------------------------
            // Left Panel File
            // --------------------------------------
            if is_submode(content, SubMode::REditorLeftPanel) {
                ui.vertical(|ui| {
                    ui.set_max_width(content.screen.leftpanel_width);

                    r_ad_leftpanel_display(content, ui);
                    leftpanel_filetree_ui(content, ui);
                });

                // Resizeable ui.separator()
                separator_left_panel(&mut content.screen, ui);
            }

            ui.vertical(|ui| {
                //    let desired_width = ( desired_width - 5.0 ).at_least(0.0);
                ui.set_max_width(desired_width);

                ScrollArea::vertical()
                    .id_source("r_editor_scroll_vertical")
                    .auto_shrink([false, false])
                    .hscroll(false)
                    .vscroll(true)
                    .scroll_bar_visibility(scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .max_height(max_height)
                    .max_width(desired_width)
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            ui.set_max_height(max_height);

                            // --------------------------------------
                            // Count Line Number
                            // --------------------------------------
                            if is_submode(content, SubMode::REditorLeftCounter) {
                                r_count_line_display(
                                    &mut content.screen,
                                    ui,
                                    &mut content.editor,
                                    font_id.clone(),
                                );
                            }

                            let text_view_width = (desired_width - 10.0).at_least(0.0);
                            //        let text_view_width = desired_width - 10.0;

                            ScrollArea::both()
                                .id_source("r_editor_scroll_horizontal")
                                .auto_shrink([false, false])
                                .hscroll(true)
                                .vscroll(false)
                                .scroll_bar_visibility(
                                    scroll_area::ScrollBarVisibility::AlwaysVisible,
                                )
                                .max_height(max_height)
                                .max_width(text_view_width)
                                .show(ui, |ui| {
                                    // --------------------------------------
                                    // R-EDITOR Basic
                                    // --------------------------------------
                                    if is_basicmode(content, BasicMode::REditorBasic) {
                                        r_editor_basic_ui(content, ui, max_height);
                                    }

                                    // --------------------------------------
                                    // R-EDITOR Normal
                                    // --------------------------------------
                                    if is_basicmode(content, BasicMode::REditorNormal) {
                                        r_editor_basic_ui(content, ui, max_height);
                                        //                r_editor_normal_ui(content, ui, max_height);
                                    }

                                    // --------------------------------------
                                    // R-EDITOR EasyMark
                                    // --------------------------------------
                                    if is_basicmode(content, BasicMode::REditorEasymark) {}

                                    ui.label(
                                        // spacing adjustment : ui.horizontal() Scroll BUG FIX
                                        egui::RichText::new("  ")
                                            .font(font_id.clone())
                                            .color(temp_color),
                                    );
                                }); // ScrollArea::both(), hscroll(true) : editor_view
                        }); // ui.horizontal() : count_line, editor_view
                    }); // ScrollArea::vertical() : editor_view_vertical - count_line, editor_view
            }); // ui.vertical() : count_line, editor_view

            // --------------------------------------
            // Right Panel Mini View
            // --------------------------------------
            if is_submode(content, SubMode::REditorRightPanel) {
                // Resizeable ui.separator()
                separator_right_panel(&mut content.screen, ui);

                ui.vertical(|ui| {
                    r_ad_rightpanel_display(content, ui);

                    let _output = ScrollArea::both()
                        .id_source("right_micro_view")
                        .vscroll(true)
                        .hscroll(true)
                        .auto_shrink([false, false])
                        .max_height(max_height)
                        .max_width((rightpanel_microview_width - 5.0).at_least(0.0))
                        // .min_scrolled_width( rightpanel_microview_width )
                        .show(ui, |ui| {
                            micro_view_ui(content, ui);
                            /*
                            let text_edit_id = output.response.id;
                            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                                let index = content_position_move_line(content, -move_row);
                                let ccursor = egui::text::CCursor::new( index );
                                state.set_ccursor_range( Some(egui::text::CCursorRange::one(ccursor)) );
                                state.store(ui.ctx(), text_edit_id);
                            }
                            let move_page_size = row_height * move_row as f32;
                            ui.scroll_with_delta( egui::vec2(0.0, move_page_size) );
                            */
                        }); // ScrollArea::both() : right_micro_view
                }); // ui.vertical()
            }
        }); // ui.horizontal_top() : line_number, editor_view, status_bar, Right Panel

        // ------------------------------------------
        // Status Bar
        // ------------------------------------------
        ui.vertical(|ui| {
            ui.separator();
            r_editor_status_bar_ui(content, ui);
        }); // ui.vertical()
    }); // ui.vertical() : ALL

    // ------------------------------------------
    // if Changed text
    // ------------------------------------------
    if content.editor.text.trim() != "" && content.editor.text_len != content.editor.text.len() {
        content.editor.text_changed = true;
        content.editor.text_len = content.editor.text.len();
        //        content.editor.freedom_request_ok = false;
    }

    if content.editor.text_changed == true && content.editor.text.trim() == "" {
        content.editor.text_changed = false;
    }
}

// ----------------------------------------------------------------------------
// R-EDITOR Status Bar
// ----------------------------------------------------------------------------

pub fn r_editor_status_bar_ui(content: &mut Content, ui: &mut egui::Ui) {
    let font_size = content.screen.font_size_normal;
    let font_id = FontId::monospace(font_size);

    let selected_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Literal as usize],
    );
    let strong_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Struct as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Struct as usize],
    );
    let normal_text_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize]
            [TokenType::StringLiteral as usize],
    );
    let unsave_color = color32_dark_light(
        content,
        content.screen.themecolor_dark_vec[Theme::MainMenu as usize][TokenType::Numeric as usize],
        content.screen.themecolor_light_vec[Theme::MainMenu as usize][TokenType::Numeric as usize],
    );
    let mark_color = color32_dark_light(content, Color32::GRAY, Color32::DARK_GRAY);
    let mut temp_color = normal_text_color;

    ui.horizontal(|ui| {
        let mut addspace = 10.0;
        if is_submode(content, SubMode::REditorLeftPanel) {
            addspace += content.screen.leftpanel_width + 20.0;
        }
        ui.add_space(addspace);

        temp_color = selected_color;
        ui.label(
            egui::RichText::new(&content.editor.filename)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(content, "current-filename:help"));

        temp_color = mark_color;
        if content.editor.text_changed {
            temp_color = unsave_color;
        }
        let change_mark: String = get_text(content, "change-mark:label");
        if ui
            .selectable_label(
                false,
                RichText::new(change_mark)
                    .font(font_id.clone())
                    .color(temp_color),
            )
            .on_hover_text(get_text(content, "changed-text:help"))
            .clicked()
        {
            submode_on(content, SubMode::REditorFileSave);
        }

        temp_color = normal_text_color;

        let byte_text = match content.editor.text_len > 1 {
            true => "bytes",
            false => "byte",
        };
        let temp_text: String = format!(
            "{:>3} {}",
            format_number_with_commas(content.editor.text_len as u64),
            byte_text
        );
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(content, "total-size:help"));

        ui.separator();

        ui.add_space(20.0);

        let temp_text: String = format!("Ln {:2}", content.editor.current_line);
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(strong_color),
        )
        .on_hover_text(get_text(content, "line-column:help"));

        let temp_text: String = format!("/{:2}, ", content.editor.total_lines);
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(content, "line-column:help"));

        let temp_text: String = format!("Col {:2}", content.editor.current_column);
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(strong_color),
        )
        .on_hover_text(get_text(content, "line-column:help"));

        let temp_text: String = format!("/{:2}", content.editor.total_columns);
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(content, "line-column:help"));

        ui.add_space(20.0);

        ui.separator();

        let temp_text: String = format!("UTF-8");
        let utf8_response = ui
            .selectable_label(
                false,
                egui::RichText::new(temp_text)
                    .font(font_id.clone())
                    .color(temp_color),
            )
            .on_hover_text(get_text(
                content,
                "Only UTF-8. Right-Click : EUR-KR ←→ UTF-8.",
            ));
        if utf8_response.secondary_clicked() {
            editor_text_byte_ansi_to_utf8(content);
        }

        ui.separator();

        let temp_text: String = format!("LF");
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(content, "Only LF"));

        ui.separator();

        let temp_text: String = format!("Sp:{}", text::TAB_SIZE);
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(
            content,
            "Auto-adjusting indentation by 1 to 4 Spaces on TAB",
        ));

        ui.separator();

        let language = get_ext_string(content.editor.filename.clone());
        let langfile_mode: bool = ext_is_languege_file(&language);
        let console_mode: bool = ext_is_console_mode(&language);

        let temp_text: String = if langfile_mode {
            format!("Language")
        } else if console_mode {
            format!("Console")
        } else {
            format!("Text")
        };
        ui.label(
            egui::RichText::new(temp_text)
                .font(font_id.clone())
                .color(temp_color),
        )
        .on_hover_text(get_text(
            content,
            "By .ext : Language Mode / Text Mode / Console Mode",
        ));
    });
}

// ----------------------------------------------------------------------------

#[inline]
pub fn min_max_clamp_f32(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}

// ----------------------------------------------------------------------------

#[inline]
pub fn utf8_byte_count(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        count += c.len_utf8();
    }
    count
}

#[inline]
pub fn utf8_col_count(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c == '\t' {
            count += text::TAB_SIZE;
        } else {
            // The simple( not exact ) width :
            // let width = match c.len_utf8() > 2 {
            //     true => 2,
            //     false => 1,
            // };

            // The exact width :
            let width = unicode_width::UnicodeWidthChar::width_cjk(c).unwrap_or(2);
            count += width;
        }
    }
    count
}

// ----------------------------------------------------------------------------
// Remove trim from each line
// ----------------------------------------------------------------------------
/*
pub fn clear_space_end_of_lines_process(text: &mut String, clear_trim: bool) {

    let save_end_of_newline = match text.chars().last() {
        Some(c) if c == '\n' => true,
        _ => false,
    };

    // Split the text into lines, but keep the line endings
    let lines_with_endings: Vec<_> = text.split_inclusive('\n').collect();

    // Remove trim from each line
    let mut lines: Vec<String> = lines_with_endings
        .iter()
        .map(|line| {
            if line.ends_with('\n') {
                line.trim_end().to_string()
            } else {
                line.to_string()
            }
        })
        .collect();
    let total_lines = lines.len();


    // Remove blank lines at the end of the text
    let mut last_line = total_lines;
    while last_line > 0 {
        let line_empty: bool = if clear_trim {
            lines[last_line - 1].trim().is_empty()
        } else {
            lines[last_line - 1].is_empty()
        };

        if line_empty {
            lines.pop();
            last_line -= 1;
        } else {
            break;
        }
    }

    let mut result_text = lines.join("\n");
    if save_end_of_newline {
        result_text.push_str("\n");
    }

    text.clear();
    text.push_str(&result_text);
}
*/
// ----------------------------------------------------------------------------

pub fn clear_space_end_of_lines(text: &mut String) {
    // Remove trim from each line
    // clear_space_end_of_lines_process(text, false);
    clear_freedom_text_process(text, 1, 1, false, false);
}

// ----------------------------------------------------------------------------

pub fn clear_freedom_end_of_lines(text: &mut String) {
    // Remove trim from each line
    // clear_space_end_of_lines_process(text, true);
    clear_freedom_text_process(text, 1, 1, false, true);
}

// ----------------------------------------------------------------------------

pub fn clear_freedom_textdata_for_work(content: &mut Content) {
    clear_freedom_textdata_process(content, true, false);
}

// ----------------------------------------------------------------------------

pub fn clear_freedom_textdata(content: &mut Content) {
    clear_freedom_textdata_process(content, false, true);
}

// ----------------------------------------------------------------------------

pub fn clear_freedom_textdata_process(
    content: &mut Content,
    is_skip_current_line: bool,
    is_remove_freedom_row: bool,
) {
    let current_line = content.editor.current_line.clone();
    let freedom_row = content.editor.freedom_row.clone();

    clear_freedom_text_process(
        &mut content.editor.text,
        current_line,
        freedom_row,
        is_skip_current_line,
        is_remove_freedom_row,
    );

    content.editor.text_len = content.editor.text.len();
    count_textdata(&mut content.editor);
    content.editor.freedom_request_ok = false;
}

// ----------------------------------------------------------------------------

pub fn clear_freedom_text_process(
    text: &mut String,
    current_line: usize,
    freedom_row: usize,
    is_skip_current_line: bool,
    is_remove_freedom_row: bool,
) {
    let save_end_of_newline = match text.chars().last() {
        Some(c) if c == '\n' => true,
        _ => false,
    };

    // Remove trim from each line
    let mut lines: Vec<String> = text
        .lines()
        .enumerate()
        .map(|(line_count, line_str)| {
            let is_current_line: bool = line_count == current_line - 1;
            if is_current_line && is_skip_current_line {
                line_str.to_string()
            } else {
                line_str.trim_end().to_string()
            }
        })
        .collect();

    if is_remove_freedom_row {
        let total_lines = lines.len();

        // Remove blank lines at the end of the document
        let mut last_line = total_lines;
        while last_line > freedom_row {
            if lines[last_line - 1].is_empty() {
                lines.pop();
                last_line -= 1;
            } else {
                break;
            }
        }
    }

    //    if content.editor.current_line > last_line {            // Bug Fix for count_textdata()
    //        content.editor.current_line = last_line;
    //    }

    *text = lines.join("\n");

    if save_end_of_newline {
        text.push_str("\n");
    }
    /*
        // Insert New Line - my favorite
        if !content.editor.text.trim().is_empty() {
            content.editor.text.push_str("\n");
        }
    */
}

// ----------------------------------------------------------------------------

pub fn add_freedom_textdata(content: &mut Content) {
    let max_column: usize = get_text(content, "text-max-column:setup")
        .parse()
        .unwrap_or(1000);

    // Insert New Line - my favorite & bug fetch
    if !content.editor.text.ends_with('\n') {
        content.editor.text.push('\n');
    }

    // Set the maximum line length as max_column.
    let mut lines: Vec<String> = Vec::new();
    for line in content.editor.text.lines() {
        if line.is_empty() {
            lines.push(line.to_string());
        } else {
            let mut remaining = line;
            while !remaining.is_empty() {
                let (chunk, rest) = if utf8_col_count(remaining) <= max_column {
                    (remaining.to_string(), "")
                } else {
                    let mut split_point = max_column;
                    for (i, c) in remaining.char_indices() {
                        if utf8_col_count(&remaining[..i + c.len_utf8()]) > max_column {
                            break;
                        }
                        split_point = i + c.len_utf8();
                    }
                    (
                        remaining[..split_point].to_string(),
                        &remaining[split_point..],
                    )
                };
                lines.push(chunk.to_string());
                remaining = rest;
            }
        }
    }

    let total_lines = lines.len();
    let freedom_row = content.editor.freedom_row;
    let mut column = content.editor.freedom_column;
    if column > max_column {
        column = max_column;
    }

    if freedom_row > total_lines {
        let num_empty_lines = freedom_row - total_lines;
        let empty_lines = std::iter::repeat("".to_string()).take(num_empty_lines);
        lines.extend(empty_lines);
    }

    for line in lines.iter_mut() {
        let current_len = utf8_col_count(line);
        let spaces = " ".repeat(column.saturating_sub(current_len));
        line.push_str(&spaces);
    }

    content.editor.text = lines.join("\n");
    // Insert New Line - my favorite
    if !content.editor.text.trim().is_empty() {
        content.editor.text.push_str("\n");
    }

    content.editor.text_len = content.editor.text.len();
    count_textdata(&mut content.editor);
    content.editor.freedom_request_ok = true;
}

// ----------------------------------------------------------------------------

pub fn clear_textdata_sub(textedit: &mut TextEditData) {
    textedit.text.clear();
    textedit.text_len = 0;
    textedit.text_byte.clear();

    textedit.text_changed = false;
    textedit.current_line = 0;
    //    textedit.ccursor.index = 0;         // Don't use it. ( use BUG : flicker 1 line number )
    //    content.editor.freedom_request_ok = false;
}

// ----------------------------------------------------------------------------

pub fn clear_textdata(textedit: &mut TextEditData) {
    clear_textdata_sub(textedit);
    textedit.ccursor.index = 0;
}

// ----------------------------------------------------------------------------

pub fn clear_textdata_reditor(content: &mut Content) {
    clear_textdata(&mut content.editor);
    // submode_on(content, SubMode::ClearUndoerReditor);
}

// ----------------------------------------------------------------------------

pub fn create_byte_line_indexes(text: &str) -> Vec<usize> {
    let (_char_line_indexes, byte_line_indexes) = create_line_indexes_from_text(text);
    byte_line_indexes
}
// ----------------------------------------------------------------------------

pub fn create_char_line_indexes(text: &str) -> Vec<usize> {
    let (char_line_indexes, _byte_line_indexes) = create_line_indexes_from_text(text);
    char_line_indexes
}

// ----------------------------------------------------------------------------

pub fn create_line_indexes_from_text(text: &str) -> (Vec<usize>, Vec<usize>) {
    let mut char_line_indexes = vec![0];
    let mut byte_line_indexes = vec![0];

    let mut char_count = 0;
    let mut byte_count = 0;

    // for (_i, c) in text.chars().enumerate() {
    for c in text.chars() {
        char_count += 1;
        byte_count += c.len_utf8();

        if c == '\n' {
            char_line_indexes.push(char_count);
            byte_line_indexes.push(byte_count);
        }
    }
    char_line_indexes.push(char_count);
    byte_line_indexes.push(byte_count);

    (char_line_indexes, byte_line_indexes)
}

// ----------------------------------------------------------------------------

pub fn get_line_start_index(line_indexes: &mut [usize], index: usize) -> usize {
    let (_current_line, line_start_index) = get_line_and_start_index(line_indexes, index);
    line_start_index
}

// ----------------------------------------------------------------------------

pub fn get_line_and_start_index(line_indexes: &mut [usize], index: usize) -> (usize, usize) {
    let mut current_line = 1;
    for (i, line_start) in line_indexes.iter().enumerate() {
        current_line = i;
        if *line_start > index {
            break;
        }
    }

    (current_line, line_indexes[current_line - 1])
}

// ----------------------------------------------------------------------------
// Content Cursor Position Count
// ----------------------------------------------------------------------------

pub fn count_textdata(textedit: &mut TextEditData) {
    let text_len = textedit.text.len();
    if text_len == 0 {
        clear_textdata_sub(textedit);
    }
    // Don't use : If you use this, text_changed will not be noticed.
    // textedit.text_len = text_len;

    let text = &textedit.text;
    let index = textedit.ccursor.index;

    let mut char_line_indexes: Vec<usize> = create_char_line_indexes(text);

    let (current_column, current_line, total_columns, total_lines) =
        get_current_column_line_total_column_line(text, &mut char_line_indexes, index);

    textedit.char_line_indexes = char_line_indexes;
    textedit.current_column = current_column;
    textedit.total_columns = total_columns;
    textedit.current_line = current_line;
    textedit.total_lines = total_lines;
}

// ----------------------------------------------------------------------------

pub fn get_current_column(text: &str, line_indexes: &mut [usize], index: usize) -> usize {
    let (current_column, _total_column) =
        get_current_column_total_column(text, line_indexes, index);
    current_column
}

// ----------------------------------------------------------------------------

pub fn get_current_column_total_column(
    text: &str,
    line_indexes: &mut [usize],
    index: usize,
) -> (usize, usize) {
    let (current_column, total_column, _total_line) =
        get_current_column_total_column_line(text, line_indexes, index);
    (current_column, total_column)
}

// ----------------------------------------------------------------------------

pub fn get_current_column_total_column_line(
    text: &str,
    line_indexes: &mut [usize],
    index: usize,
) -> (usize, usize, usize) {
    let (current_column, _current_line, total_columns, total_lines) =
        get_current_column_line_total_column_line(text, line_indexes, index);

    (current_column, total_columns, total_lines)
}

// ----------------------------------------------------------------------------

pub fn get_current_column_line_total_column_line(
    text: &str,
    line_indexes: &mut [usize],
    index: usize,
) -> (usize, usize, usize, usize) {
    let (current_line, line_start_index) = get_line_and_start_index(line_indexes, index);

    let total_lines = line_indexes.len() - 1;

    let mut i: usize = line_start_index;
    let mut current_column: usize = 1;
    let mut total_columns: usize = 0;
    while let Some(c) = text.chars().nth(i) {
        if c == '\n' {
            break;
        } else if c == '\t' {
            total_columns += text::TAB_SIZE;
        } else {
            // The simple( not exact ) width :
            // let width = match c.len_utf8() > 2 {
            //     true => 2,
            //     false => 1,
            // };

            // The exact width :
            let width = unicode_width::UnicodeWidthChar::width_cjk(c).unwrap_or(2);

            total_columns += width;
        }

        if i < index {
            current_column = total_columns + 1;
        }
        i += 1;
    }

    (current_column, current_line, total_columns, total_lines)
}

// ----------------------------------------------------------------------------
