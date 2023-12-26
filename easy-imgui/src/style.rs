use super::*;

impl Context {
    pub fn style(&mut self) -> StyleMut<'_> {
        let ptr = unsafe {
            &mut *ImGui_GetStyle()
        };
        StyleMut(StylePtr {
            ptr,
        })
    }
}

impl<A> Ui<A> {
    pub fn style(&self) -> Style<'_> {
        let ptr = unsafe {
            &mut *ImGui_GetStyle()
        };
        Style(StylePtr {
            ptr,
        })
    }
}

/// An immutable reference to the ImGui styles.
#[derive(Debug)]
pub struct Style<'a>(StylePtr<'a>);

/// A mutable reference to the ImGui styles.
///
/// You can't get this from a running frame [`Ui`], because you can't change the style during a
/// frame. This can only be retrieved from a mutable [`Context`].
/// To change the style during a frame use a [`Pushable`].
#[derive(Debug)]
pub struct StyleMut<'a>(StylePtr<'a>);

impl<'a> Deref for Style<'a> {
    type Target = StylePtr<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Deref for StyleMut<'a> {
    type Target = StylePtr<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StyleMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct StylePtr<'a> {
    ptr: &'a mut ImGuiStyle,
}

impl<'a> StylePtr<'a> {
    pub fn set_colors_light(&mut self) {
        unsafe {
            ImGui_StyleColorsLight(self.ptr);
        }
    }
    pub fn set_colors_dark(&mut self) {
        unsafe {
            ImGui_StyleColorsDark(self.ptr);
        }
    }
    pub fn set_colors_classic(&mut self) {
        unsafe {
            ImGui_StyleColorsClassic(self.ptr);
        }
    }
    pub fn color(&self, id: ColorId) -> Color {
        self.ptr.Colors[id.bits() as usize].into()
    }
    pub fn alpha(&self) -> f32 {
        self.ptr.Alpha
    }
    pub fn set_color(&mut self, id: ColorId, color: Color) {
        self.ptr.Colors[id.bits() as usize] = color.into();
    }
    pub fn set_alpha(&mut self, alpha: f32) {
        self.ptr.Alpha = alpha;
    }
    pub fn color_alpha(&self, id: ColorId, alpha_mul: f32) -> Color {
        let mut c = self.color(id);
        let a = self.alpha();
        c.a *= a * alpha_mul;
        c
    }
    pub fn frame_padding(&self) -> Vector2 {
        self.ptr.FramePadding.into()
    }
    pub fn frame_rounding(&self) -> f32 {
        self.ptr.FrameRounding
    }
    pub fn frame_border_size(&self) -> f32 {
        self.ptr.FrameBorderSize
    }
    pub fn item_spacing(&self) -> Vector2 {
        self.ptr.ItemSpacing.into()
    }
    pub fn item_inner_spacing(&self) -> Vector2 {
        self.ptr.ItemInnerSpacing.into()
    }
}
