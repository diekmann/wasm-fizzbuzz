use std::collections::VecDeque;
use std::os::raw::c_int;
use std::sync::Mutex;

#[no_mangle]
extern "C" fn I_InitGraphics() {
    crate::log!("I_InitGraphics (TODO)");
}

#[no_mangle]
extern "C" fn I_StartFrame() {
    // From the original Doom Sources:
    // https://github.com/id-Software/DOOM/blob/master/linuxdoom-1.10/i_video.c#L185
    // // er?
}

// d_event.h: evtype_t
#[repr(C)]
pub enum EventType {
    KeyDown,
    KeyUp,
    Mouse,
    Joystick,
}

// d_event.h: event_t
#[repr(C)]
struct Event {
    evtype: EventType,
    data1: c_int, // keys / mouse/joystick buttons
    data2: c_int, // mouse/joystick x move
    data3: c_int, // mouse/joystick y move
}

// Shared queue where JavaScript may asynchronously place events, such as keypresses.
lazy_static! {
    static ref INPUT_EVENT_QUEUE: Mutex<VecDeque<Event>> = Mutex::from(VecDeque::new());
}

#[no_mangle]
pub extern "C" fn add_browser_event(evtype: EventType, data1: i32) {
    let mut q = INPUT_EVENT_QUEUE
        .lock()
        .expect("INPUT_EVENT_QUEUE locking failed (called from JS to add events)");
    q.push_back(Event {
        evtype,
        data1,
        data2: 0,
        data3: 0,
    })

    // TODO: also support mouse.
}

extern "C" {
    fn D_PostEvent(ev: *const Event);
}

#[no_mangle]
extern "C" fn I_StartTic() {
    // should get inputs (e.g. key presses/releases)
    // and send them to D_PostEvent().

    fn post_event(ev: &Event) {
        unsafe {
            D_PostEvent(ev);
        }
    };

    let mut q = INPUT_EVENT_QUEUE
        .lock()
        .expect("INPUT_EVENT_QUEUE locking failed (called from rust to consume events)");
    while let Some(ev) = q.pop_front() {
        post_event(&ev);
    }
}

#[no_mangle]
extern "C" fn I_UpdateNoBlit() {
    // From the original Doom sources:
    // https://github.com/id-Software/DOOM/blob/master/linuxdoom-1.10/i_video.c#L346
    // // what is this?
}


extern "C" {
    static usegamma: c_int;
    static gammatable: [[u8; 256]; 5];
}

struct Palette<'a>(&'a [u8]);

impl<'a> Palette<'a> {
    unsafe fn from(palette: *const u8) -> Self {
        Palette(std::slice::from_raw_parts(palette, 256*3))
    }

    fn into_iter(self) -> PaletteIter<'a> {
        PaletteIter(self.0.into_iter())
    }
}

struct PaletteIter<'a>(std::slice::Iter<'a, u8>);

impl Iterator for PaletteIter<'_> {
    type Item = (u8, u8, u8);

    fn next(&mut self) -> Option<(u8, u8, u8)>{
        let r = match self.0.next() {
            Some(r) => *r,
            None => return None,
        };
        let g = match self.0.next() {
            Some(g) => *g,
            None => return None,
        };
        let b = match self.0.next() {
            Some(b) => *b,
            None => return None,
        };
        Some((r, g, b))
    }
}

#[no_mangle]
extern "C" fn I_SetPalette(palette: *const u8) {
    let palette = unsafe { Palette::from(palette) };
    let gt = unsafe{ gammatable[usegamma as usize] };
    for (i, (r, g, b)) in palette.into_iter().enumerate() {
        let r = gt[r as usize];
        let g = gt[g as usize];
        let b = gt[b as usize];
        let color = RGBAColor(r, g, b, 255);
        unsafe { COLORMAP[i] = color };
    }
}

// C libraries
extern "C" {
    // v_video.h where doom puts its rendered graphics.
    static screens: [*const u8; 5];
}

// The X11 color used by doom.
struct XColor(/*red*/ u16, /*green*/ u16, /*blue*/ u16);

// The color we use fomr JavaScript to render to the HTML5 canvas.
#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy)]
struct RGBAColor(
    /*red*/ u8,
    /*green*/ u8,
    /*blue*/ u8,
    /*alpha*/ u8,
);

#[link(wasm_import_module = "js")]
extern "C" {
    // ptr points to a SCREENWIDTH*SCREENHEIGHT array.
    fn js_draw_screen(ptr: *const RGBAColor);
}

impl XColor {
    const fn to_rgba(self) -> RGBAColor {
        let XColor(r, g, b) = self;
        let opaque = 255;
        const fn _scale(c: u16) -> u8 {
            (c / 256) as u8
        }
        RGBAColor(_scale(r), _scale(g), _scale(b), opaque)
    }
}

// Each byte in doom's screen represents one 256-color PseudoColor Pixel.
// This needs to be mapped to the real color. Doom has its own mapping.
// Generated in i_video.c in UploadNewPalette via
// for (i=0 ; i<256 ; i++){printf("XColor(%hu, %hu, %hu).to_rgba(),\n", colors[i].red, colors[i].green, colors[i].blue);}
static mut COLORMAP: [RGBAColor; 256] = [
    XColor(257, 257, 257).to_rgba(),
    XColor(8224, 6168, 3084).to_rgba(),
    XColor(6168, 4112, 2056).to_rgba(),
    XColor(19532, 19532, 19532).to_rgba(),
    XColor(65535, 65535, 65535).to_rgba(),
    XColor(7196, 7196, 7196).to_rgba(),
    XColor(5140, 5140, 5140).to_rgba(),
    XColor(3084, 3084, 3084).to_rgba(),
    XColor(2056, 2056, 2056).to_rgba(),
    XColor(12336, 14392, 8224).to_rgba(),
    XColor(9252, 11308, 4112).to_rgba(),
    XColor(6168, 8224, 2056).to_rgba(),
    XColor(4112, 6168, 257).to_rgba(),
    XColor(20560, 15420, 11308).to_rgba(),
    XColor(18504, 13364, 9252).to_rgba(),
    XColor(16448, 11308, 7196).to_rgba(),
    XColor(65535, 47031, 47031).to_rgba(),
    XColor(63479, 43947, 43947).to_rgba(),
    XColor(62451, 41891, 41891).to_rgba(),
    XColor(60395, 38807, 38807).to_rgba(),
    XColor(59367, 36751, 36751).to_rgba(),
    XColor(57311, 34695, 34695).to_rgba(),
    XColor(56283, 31868, 31868).to_rgba(),
    XColor(54227, 29812, 29812).to_rgba(),
    XColor(52171, 27756, 27756).to_rgba(),
    XColor(51143, 25700, 25700).to_rgba(),
    XColor(49087, 23644, 23644).to_rgba(),
    XColor(48059, 22616, 22616).to_rgba(),
    XColor(46003, 20560, 20560).to_rgba(),
    XColor(44975, 18504, 18504).to_rgba(),
    XColor(42919, 16448, 16448).to_rgba(),
    XColor(41891, 15420, 15420).to_rgba(),
    XColor(39835, 13364, 13364).to_rgba(),
    XColor(38807, 12336, 12336).to_rgba(),
    XColor(36751, 11308, 11308).to_rgba(),
    XColor(35723, 9252, 9252).to_rgba(),
    XColor(33667, 8224, 8224).to_rgba(),
    XColor(32896, 7196, 7196).to_rgba(),
    XColor(30840, 6168, 6168).to_rgba(),
    XColor(29812, 5140, 5140).to_rgba(),
    XColor(27756, 4112, 4112).to_rgba(),
    XColor(26728, 3084, 3084).to_rgba(),
    XColor(24672, 2056, 2056).to_rgba(),
    XColor(23644, 2056, 2056).to_rgba(),
    XColor(21588, 2056, 2056).to_rgba(),
    XColor(20560, 257, 257).to_rgba(),
    XColor(18504, 257, 257).to_rgba(),
    XColor(17476, 257, 257).to_rgba(),
    XColor(65535, 60395, 57311).to_rgba(),
    XColor(65535, 58339, 54227).to_rgba(),
    XColor(65535, 56283, 51143).to_rgba(),
    XColor(65535, 54227, 48059).to_rgba(),
    XColor(65535, 53199, 46003).to_rgba(),
    XColor(65535, 51143, 42919).to_rgba(),
    XColor(65535, 49087, 39835).to_rgba(),
    XColor(65535, 48059, 37779).to_rgba(),
    XColor(65535, 46003, 33667).to_rgba(),
    XColor(63479, 43947, 31868).to_rgba(),
    XColor(61423, 41891, 29812).to_rgba(),
    XColor(59367, 39835, 27756).to_rgba(),
    XColor(57311, 37779, 25700).to_rgba(),
    XColor(55255, 35723, 23644).to_rgba(),
    XColor(53199, 33667, 21588).to_rgba(),
    XColor(52171, 32896, 20560).to_rgba(),
    XColor(49087, 31868, 19532).to_rgba(),
    XColor(46003, 29812, 18504).to_rgba(),
    XColor(43947, 28784, 17476).to_rgba(),
    XColor(41891, 27756, 16448).to_rgba(),
    XColor(39835, 25700, 15420).to_rgba(),
    XColor(36751, 24672, 14392).to_rgba(),
    XColor(34695, 22616, 13364).to_rgba(),
    XColor(32896, 21588, 12336).to_rgba(),
    XColor(30840, 20560, 11308).to_rgba(),
    XColor(27756, 18504, 10280).to_rgba(),
    XColor(24672, 17476, 9252).to_rgba(),
    XColor(21588, 16448, 8224).to_rgba(),
    XColor(19532, 14392, 7196).to_rgba(),
    XColor(16448, 12336, 6168).to_rgba(),
    XColor(13364, 11308, 5140).to_rgba(),
    XColor(11308, 9252, 4112).to_rgba(),
    XColor(61423, 61423, 61423).to_rgba(),
    XColor(59367, 59367, 59367).to_rgba(),
    XColor(57311, 57311, 57311).to_rgba(),
    XColor(56283, 56283, 56283).to_rgba(),
    XColor(54227, 54227, 54227).to_rgba(),
    XColor(52171, 52171, 52171).to_rgba(),
    XColor(51143, 51143, 51143).to_rgba(),
    XColor(49087, 49087, 49087).to_rgba(),
    XColor(47031, 47031, 47031).to_rgba(),
    XColor(46003, 46003, 46003).to_rgba(),
    XColor(43947, 43947, 43947).to_rgba(),
    XColor(42919, 42919, 42919).to_rgba(),
    XColor(40863, 40863, 40863).to_rgba(),
    XColor(38807, 38807, 38807).to_rgba(),
    XColor(37779, 37779, 37779).to_rgba(),
    XColor(35723, 35723, 35723).to_rgba(),
    XColor(33667, 33667, 33667).to_rgba(),
    XColor(32896, 32896, 32896).to_rgba(),
    XColor(30840, 30840, 30840).to_rgba(),
    XColor(28784, 28784, 28784).to_rgba(),
    XColor(27756, 27756, 27756).to_rgba(),
    XColor(25700, 25700, 25700).to_rgba(),
    XColor(23644, 23644, 23644).to_rgba(),
    XColor(22616, 22616, 22616).to_rgba(),
    XColor(20560, 20560, 20560).to_rgba(),
    XColor(18504, 18504, 18504).to_rgba(),
    XColor(17476, 17476, 17476).to_rgba(),
    XColor(15420, 15420, 15420).to_rgba(),
    XColor(14392, 14392, 14392).to_rgba(),
    XColor(12336, 12336, 12336).to_rgba(),
    XColor(10280, 10280, 10280).to_rgba(),
    XColor(9252, 9252, 9252).to_rgba(),
    XColor(30840, 65535, 28784).to_rgba(),
    XColor(28784, 61423, 26728).to_rgba(),
    XColor(26728, 57311, 24672).to_rgba(),
    XColor(24672, 53199, 22616).to_rgba(),
    XColor(23644, 49087, 20560).to_rgba(),
    XColor(21588, 44975, 18504).to_rgba(),
    XColor(19532, 40863, 16448).to_rgba(),
    XColor(17476, 37779, 14392).to_rgba(),
    XColor(16448, 33667, 12336).to_rgba(),
    XColor(14392, 29812, 11308).to_rgba(),
    XColor(12336, 25700, 9252).to_rgba(),
    XColor(10280, 21588, 7196).to_rgba(),
    XColor(8224, 17476, 6168).to_rgba(),
    XColor(6168, 13364, 4112).to_rgba(),
    XColor(5140, 9252, 3084).to_rgba(),
    XColor(3084, 6168, 2056).to_rgba(),
    XColor(49087, 42919, 36751).to_rgba(),
    XColor(47031, 40863, 34695).to_rgba(),
    XColor(44975, 38807, 32896).to_rgba(),
    XColor(42919, 36751, 30840).to_rgba(),
    XColor(40863, 34695, 28784).to_rgba(),
    XColor(39835, 32896, 27756).to_rgba(),
    XColor(37779, 31868, 25700).to_rgba(),
    XColor(35723, 29812, 23644).to_rgba(),
    XColor(33667, 27756, 22616).to_rgba(),
    XColor(31868, 25700, 20560).to_rgba(),
    XColor(30840, 24672, 19532).to_rgba(),
    XColor(28784, 22616, 17476).to_rgba(),
    XColor(26728, 21588, 16448).to_rgba(),
    XColor(24672, 19532, 14392).to_rgba(),
    XColor(22616, 17476, 13364).to_rgba(),
    XColor(21588, 16448, 12336).to_rgba(),
    XColor(40863, 33667, 25700).to_rgba(),
    XColor(36751, 30840, 21588).to_rgba(),
    XColor(33667, 27756, 19532).to_rgba(),
    XColor(30840, 24672, 16448).to_rgba(),
    XColor(26728, 21588, 13364).to_rgba(),
    XColor(23644, 18504, 11308).to_rgba(),
    XColor(20560, 15420, 9252).to_rgba(),
    XColor(17476, 13364, 7196).to_rgba(),
    XColor(31868, 32896, 25700).to_rgba(),
    XColor(28784, 29812, 22616).to_rgba(),
    XColor(26728, 27756, 20560).to_rgba(),
    XColor(23644, 25700, 18504).to_rgba(),
    XColor(21588, 22616, 15420).to_rgba(),
    XColor(18504, 20560, 13364).to_rgba(),
    XColor(16448, 18504, 11308).to_rgba(),
    XColor(14392, 16448, 10280).to_rgba(),
    XColor(65535, 65535, 29812).to_rgba(),
    XColor(60395, 56283, 22616).to_rgba(),
    XColor(55255, 48059, 17476).to_rgba(),
    XColor(50115, 39835, 12336).to_rgba(),
    XColor(44975, 31868, 8224).to_rgba(),
    XColor(39835, 23644, 5140).to_rgba(),
    XColor(34695, 17476, 2056).to_rgba(),
    XColor(29812, 11308, 257).to_rgba(),
    XColor(65535, 65535, 65535).to_rgba(),
    XColor(65535, 56283, 56283).to_rgba(),
    XColor(65535, 48059, 48059).to_rgba(),
    XColor(65535, 39835, 39835).to_rgba(),
    XColor(65535, 31868, 31868).to_rgba(),
    XColor(65535, 24672, 24672).to_rgba(),
    XColor(65535, 16448, 16448).to_rgba(),
    XColor(65535, 8224, 8224).to_rgba(),
    XColor(65535, 257, 257).to_rgba(),
    XColor(61423, 257, 257).to_rgba(),
    XColor(58339, 257, 257).to_rgba(),
    XColor(55255, 257, 257).to_rgba(),
    XColor(52171, 257, 257).to_rgba(),
    XColor(49087, 257, 257).to_rgba(),
    XColor(46003, 257, 257).to_rgba(),
    XColor(42919, 257, 257).to_rgba(),
    XColor(39835, 257, 257).to_rgba(),
    XColor(35723, 257, 257).to_rgba(),
    XColor(32896, 257, 257).to_rgba(),
    XColor(29812, 257, 257).to_rgba(),
    XColor(26728, 257, 257).to_rgba(),
    XColor(23644, 257, 257).to_rgba(),
    XColor(20560, 257, 257).to_rgba(),
    XColor(17476, 257, 257).to_rgba(),
    XColor(59367, 59367, 65535).to_rgba(),
    XColor(51143, 51143, 65535).to_rgba(),
    XColor(43947, 43947, 65535).to_rgba(),
    XColor(36751, 36751, 65535).to_rgba(),
    XColor(29812, 29812, 65535).to_rgba(),
    XColor(21588, 21588, 65535).to_rgba(),
    XColor(14392, 14392, 65535).to_rgba(),
    XColor(7196, 7196, 65535).to_rgba(),
    XColor(257, 257, 65535).to_rgba(),
    XColor(257, 257, 58339).to_rgba(),
    XColor(257, 257, 52171).to_rgba(),
    XColor(257, 257, 46003).to_rgba(),
    XColor(257, 257, 39835).to_rgba(),
    XColor(257, 257, 33667).to_rgba(),
    XColor(257, 257, 27756).to_rgba(),
    XColor(257, 257, 21588).to_rgba(),
    XColor(65535, 65535, 65535).to_rgba(),
    XColor(65535, 60395, 56283).to_rgba(),
    XColor(65535, 55255, 48059).to_rgba(),
    XColor(65535, 51143, 39835).to_rgba(),
    XColor(65535, 46003, 31868).to_rgba(),
    XColor(65535, 41891, 23644).to_rgba(),
    XColor(65535, 36751, 15420).to_rgba(),
    XColor(65535, 32896, 7196).to_rgba(),
    XColor(62451, 29812, 6168).to_rgba(),
    XColor(60395, 28784, 4112).to_rgba(),
    XColor(57311, 26728, 4112).to_rgba(),
    XColor(55255, 24672, 3084).to_rgba(),
    XColor(52171, 22616, 2056).to_rgba(),
    XColor(50115, 20560, 257).to_rgba(),
    XColor(47031, 18504, 257).to_rgba(),
    XColor(44975, 17476, 257).to_rgba(),
    XColor(65535, 65535, 65535).to_rgba(),
    XColor(65535, 65535, 55255).to_rgba(),
    XColor(65535, 65535, 46003).to_rgba(),
    XColor(65535, 65535, 36751).to_rgba(),
    XColor(65535, 65535, 27756).to_rgba(),
    XColor(65535, 65535, 18504).to_rgba(),
    XColor(65535, 65535, 9252).to_rgba(),
    XColor(65535, 65535, 257).to_rgba(),
    XColor(42919, 16448, 257).to_rgba(),
    XColor(40863, 14392, 257).to_rgba(),
    XColor(37779, 12336, 257).to_rgba(),
    XColor(34695, 9252, 257).to_rgba(),
    XColor(20560, 15420, 10280).to_rgba(),
    XColor(17476, 12336, 7196).to_rgba(),
    XColor(14392, 9252, 5140).to_rgba(),
    XColor(12336, 7196, 3084).to_rgba(),
    XColor(257, 257, 21588).to_rgba(),
    XColor(257, 257, 18504).to_rgba(),
    XColor(257, 257, 15420).to_rgba(),
    XColor(257, 257, 12336).to_rgba(),
    XColor(257, 257, 9252).to_rgba(),
    XColor(257, 257, 6168).to_rgba(),
    XColor(257, 257, 3084).to_rgba(),
    XColor(257, 257, 257).to_rgba(),
    XColor(65535, 40863, 17476).to_rgba(),
    XColor(65535, 59367, 19532).to_rgba(),
    XColor(65535, 31868, 65535).to_rgba(),
    XColor(65535, 257, 65535).to_rgba(),
    XColor(53199, 257, 53199).to_rgba(),
    XColor(40863, 257, 39835).to_rgba(),
    XColor(28784, 257, 27756).to_rgba(),
    XColor(42919, 27756, 27756).to_rgba(),
];

// The screens are SCREENWIDTH*SCREENHEIGHT, which is 320x200
const SCREENWIDTH: usize = 320;
const SCREENHEIGHT: usize = 200;
const SCREENSIZE: usize = SCREENWIDTH * SCREENHEIGHT;


#[no_mangle]
extern "C" fn I_FinishUpdate() {
    // TODO: move to testing
    assert_eq!(std::mem::size_of::<RGBAColor>(), 4); // 4 packed bytes of rgba as expected by JS.

    // Draws the screen
    // Doom's C sources define
    //
    // // Screen 0 is the screen updated by I_Update screen.
    // // Screen 1 is an extra buffer.
    // extern	byte*		screens[5];
    //
    // I think only screens[0] is needed.
    let the_screen = unsafe { std::slice::from_raw_parts(screens[0], SCREENSIZE) };

    // We make the CANVAS a static varibale, so it is not freshly allocated onto rust's
    // small stack on each call.
    // If Javascript throws a `RuntimeError: index out of bounds` and the traceback
    // shows this is inside a `memset`, it's likely because we exhausted the stack size.
    static mut CANVAS: [RGBAColor; SCREENSIZE*4] = [RGBAColor(0, 0, 0, 255); SCREENSIZE*4];

     // Double the screen size. Pixel perfect, no interpolation.
     const MULTIPLY: usize = 2;

     for y in 0..SCREENHEIGHT {
         for x in 0..SCREENWIDTH {
             let pixel = the_screen[y*SCREENWIDTH + x] as usize;
             unsafe {
                let rgba_pixel = COLORMAP[pixel];
                CANVAS[y*MULTIPLY*MULTIPLY*SCREENWIDTH + x*MULTIPLY] = rgba_pixel;
                CANVAS[y*MULTIPLY*MULTIPLY*SCREENWIDTH + x*MULTIPLY + 1] = rgba_pixel;
                CANVAS[y*MULTIPLY*MULTIPLY*SCREENWIDTH + SCREENWIDTH*MULTIPLY + x*MULTIPLY] = rgba_pixel;
                CANVAS[y*MULTIPLY*MULTIPLY*SCREENWIDTH + SCREENWIDTH*MULTIPLY + x*MULTIPLY + 1] = rgba_pixel;
             }
         }
     }

    unsafe {
        js_draw_screen(CANVAS.as_ptr());
    }
}

#[no_mangle]
extern "C" fn I_ReadScreen(scr: *mut u8) {
    // Doom does: memcpy (/*dst!!*/ scr, /*src!!*/ screens[0], SCREENWIDTH*SCREENHEIGHT);
    unsafe { std::ptr::copy_nonoverlapping(screens[0], scr, SCREENSIZE) }
}

#[no_mangle]
extern "C" fn I_ShutdownGraphics() {
    crate::log!("Bye!! TODO: implement I_ShutdownGraphics");
}
