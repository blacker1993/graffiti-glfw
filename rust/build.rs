use cc::Build;

// based on https://github.com/glfw/glfw/blob/master/src/CMakeLists.txt
fn main() {
    let mut build = Build::new();

    // no warns
    build.flag("-w");

    // define platform first
    #[cfg(target_os="macos")]
    build.define("_GLFW_COCOA", Some("1"));

    #[cfg(target_os="linux")]
    build.define("_GLFW_X11", Some("1"));

    // shared
    build
        .file("../src/context.c")
        .file("../src/init.c")
        .file("../src/input.c")
        .file("../src/monitor.c")
        .file("../src/vulkan.c")
        .file("../src/window.c")
    ;

    #[cfg(target_os="macos")]
    build
        .file("../src/cocoa_init.m")
        .file("../src/cocoa_joystick.m")
        .file("../src/cocoa_monitor.m")
        .file("../src/cocoa_window.m")
        .file("../src/cocoa_time.c")
        .file("../src/posix_thread.c")
        .file("../src/nsgl_context.m")
        .file("../src/egl_context.c")
        .file("../src/osmesa_context.c")
    ;

    #[cfg(target_os="linux")]
    build
      // TODO: wayland
      .file("../src/x11_init.c")
      .file("../src/x11_monitor.c")
      .file("../src/x11_window.c")

      .file("../src/xkb_unicode.c")
      .file("../src/posix_time.c")
      .file("../src/posix_thread.c")
      .file("../src/glx_context.c")
      .file("../src/egl_context.c")
      .file("../src/osmesa_context.c")

      .file("../src/linux_joystick.c")
    ;

    // build lib
    build
        .compile("libglfw3.a")
    ;
}
