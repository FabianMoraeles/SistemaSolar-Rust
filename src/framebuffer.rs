/// Framebuffer para renderizado por software
/// Contiene el buffer de color y el z-buffer (depth buffer)
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,   // Color buffer en formato 0xAARRGGBB
    pub zbuffer: Vec<f32>,  // Depth buffer para oclusión correcta
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con las dimensiones especificadas
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            buffer: vec![0; size],
            zbuffer: vec![f32::INFINITY; size],
        }
    }

    /// Limpia el framebuffer con un color sólido
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
        self.zbuffer.fill(f32::INFINITY);
    }

    /// Establece un píxel en la posición (x, y) con el color especificado
    /// No hace nada si las coordenadas están fuera de los límites
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    /// Establece un píxel con depth testing
    /// Solo dibuja si la profundidad z es menor que la almacenada
    pub fn set_pixel_with_depth(&mut self, x: usize, y: usize, color: u32, z: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if z < self.zbuffer[index] {
                self.buffer[index] = color;
                self.zbuffer[index] = z;
            }
        }
    }

    /// Obtiene el color de un píxel (útil para debugging)
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    /// Dibuja una línea usando el algoritmo de Bresenham
    /// Útil para renderizar órbitas y wireframes
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < self.width as i32 && y0 >= 0 && y0 < self.height as i32 {
                self.set_pixel(x0 as usize, y0 as usize, color);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Dibuja un círculo (útil para órbitas)
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;

        while y >= x {
            self.draw_circle_points(cx, cy, x, y, color);
            x += 1;

            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
        }
    }

    fn draw_circle_points(&mut self, cx: i32, cy: i32, x: i32, y: i32, color: u32) {
        let points = [
            (cx + x, cy + y), (cx - x, cy + y),
            (cx + x, cy - y), (cx - x, cy - y),
            (cx + y, cy + x), (cx - y, cy + x),
            (cx + y, cy - x), (cx - y, cy - x),
        ];

        for (px, py) in points.iter() {
            if *px >= 0 && *px < self.width as i32 && *py >= 0 && *py < self.height as i32 {
                self.set_pixel(*px as usize, *py as usize, color);
            }
        }
    }
}

/// Helper para crear colores en formato 0xAARRGGBB
#[allow(dead_code)]
pub fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Helper para interpolar colores
#[allow(dead_code)]
pub fn lerp_color(color1: u32, color2: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    
    let r1 = ((color1 >> 16) & 0xFF) as f32;
    let g1 = ((color1 >> 8) & 0xFF) as f32;
    let b1 = (color1 & 0xFF) as f32;
    
    let r2 = ((color2 >> 16) & 0xFF) as f32;
    let g2 = ((color2 >> 8) & 0xFF) as f32;
    let b2 = (color2 & 0xFF) as f32;
    
    let r = (r1 + (r2 - r1) * t) as u32;
    let g = (g1 + (g2 - g1) * t) as u32;
    let b = (b1 + (b2 - b1) * t) as u32;
    
    0xFF000000 | (r << 16) | (g << 8) | b
}