fn main() {
    let s: wgpu::Surface = unsafe { std::mem::zeroed() };
    let () = s.get_current_texture();
}
