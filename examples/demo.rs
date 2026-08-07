use iced::widget::{
    Space, button, checkbox, column, container, pick_list, progress_bar, radio, row, rule,
    scrollable, slider, stack, text, text_input, toggler,
};
use iced::{Center, Color, Element, Length, Theme};

use material_theme::{
    MaterialTheme, Mode, Variant, button as m3_button, checkbox as m3_checkbox,
    container as m3_container, menu as m3_menu, pick_list as m3_pick_list,
    progress_bar as m3_progress_bar, radio as m3_radio, rule as m3_rule, scrollable as m3_scrollable,
    slider as m3_slider, text_input as m3_text_input, toggler as m3_toggler,
};
use widgets::color_picker_two::color_picker_two;
// A third-party widget styled through the `m3_theme` feature — it tracks the
// seed color and light/dark mode along with everything else on screen.
use widgets::m3::color_picker_two::material as m3_picker;

const BASELINE_PURPLE_ARGB: u32 = 0xFF6750A4;

fn color_to_argb(c: Color) -> u32 {
    let to_u8 = |x: f32| (x.clamp(0.0, 1.0) * 255.0).round() as u32;
    (to_u8(c.a) << 24) | (to_u8(c.r) << 16) | (to_u8(c.g) << 8) | to_u8(c.b)
}

fn argb_to_color(argb: u32) -> Color {
    Color::from_rgba8(
        ((argb >> 16) & 0xFF) as u8,
        ((argb >> 8) & 0xFF) as u8,
        (argb & 0xFF) as u8,
        ((argb >> 24) & 0xFF) as f32 / 255.0,
    )
}

struct Demo {
    mode: ModeChoice,
    variant: VariantChoice,
    seed_color: Color,
    picker_open: bool,
    input_text: String,
    checkbox_a: bool,
    checkbox_b: bool,
    toggler_a: bool,
    toggler_b: bool,
    slider_value: f32,
    radio_choice: Option<RadioChoice>,
    pick_value: Option<PickChoice>,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            mode: ModeChoice::default(),
            variant: VariantChoice::default(),
            seed_color: argb_to_color(BASELINE_PURPLE_ARGB),
            picker_open: false,
            input_text: String::new(),
            checkbox_a: false,
            checkbox_b: false,
            toggler_a: false,
            toggler_b: false,
            slider_value: 0.0,
            radio_choice: None,
            pick_value: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ModeChoice {
    #[default]
    Light,
    Dark,
}

impl ModeChoice {
    fn into_mode(self) -> Mode {
        match self {
            ModeChoice::Light => Mode::Light,
            ModeChoice::Dark => Mode::Dark,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum VariantChoice {
    #[default]
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Neutral,
    Monochrome,
}

impl VariantChoice {
    fn into_variant(self) -> Variant {
        match self {
            VariantChoice::TonalSpot => Variant::TonalSpot,
            VariantChoice::Vibrant => Variant::Vibrant,
            VariantChoice::Expressive => Variant::Expressive,
            VariantChoice::Fidelity => Variant::Fidelity,
            VariantChoice::Content => Variant::Content,
            VariantChoice::Neutral => Variant::Neutral,
            VariantChoice::Monochrome => Variant::Monochrome,
        }
    }

    fn label(self) -> &'static str {
        match self {
            VariantChoice::TonalSpot => "Tonal Spot",
            VariantChoice::Vibrant => "Vibrant",
            VariantChoice::Expressive => "Expressive",
            VariantChoice::Fidelity => "Fidelity",
            VariantChoice::Content => "Content",
            VariantChoice::Neutral => "Neutral",
            VariantChoice::Monochrome => "Monochrome",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadioChoice {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickChoice {
    One,
    Two,
    Three,
}

impl std::fmt::Display for PickChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PickChoice::One => "One",
            PickChoice::Two => "Two",
            PickChoice::Three => "Three",
        })
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMode,
    SelectVariant(VariantChoice),
    InputChanged(String),
    CheckboxA(bool),
    CheckboxB(bool),
    TogglerA(bool),
    TogglerB(bool),
    SliderChanged(f32),
    RadioSelected(RadioChoice),
    PickSelected(PickChoice),
    OpenPicker,
    ClosePicker,
    SeedColorPicked(Color),
}

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleMode => {
                self.mode = match self.mode {
                    ModeChoice::Light => ModeChoice::Dark,
                    ModeChoice::Dark => ModeChoice::Light,
                };
            }
            Message::SelectVariant(v) => self.variant = v,
            Message::InputChanged(t) => self.input_text = t,
            Message::CheckboxA(v) => self.checkbox_a = v,
            Message::CheckboxB(v) => self.checkbox_b = v,
            Message::TogglerA(v) => self.toggler_a = v,
            Message::TogglerB(v) => self.toggler_b = v,
            Message::SliderChanged(v) => self.slider_value = v,
            Message::RadioSelected(r) => self.radio_choice = Some(r),
            Message::PickSelected(p) => self.pick_value = Some(p),
            Message::OpenPicker => self.picker_open = true,
            Message::ClosePicker => self.picker_open = false,
            Message::SeedColorPicked(color) => {
                self.seed_color = Color { a: 1.0, ..color };
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Material Design 3 — iced theme demo").size(28);

        let mode_btn = button(text(match self.mode {
            ModeChoice::Light => "Switch to Dark",
            ModeChoice::Dark => "Switch to Light",
        }))
        .on_press(Message::ToggleMode)
        .style(m3_button::tonal);

        let variant_row = row![
            text("Variant:").size(14),
            variant_chip(self.variant, VariantChoice::TonalSpot),
            variant_chip(self.variant, VariantChoice::Vibrant),
            variant_chip(self.variant, VariantChoice::Expressive),
            variant_chip(self.variant, VariantChoice::Fidelity),
            variant_chip(self.variant, VariantChoice::Content),
            variant_chip(self.variant, VariantChoice::Neutral),
            variant_chip(self.variant, VariantChoice::Monochrome),
        ]
        .spacing(8)
        .align_y(Center);

        let seed_swatch = container(text(""))
            .style(m3_container::swatch_box(self.seed_color, self.seed_color))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0));
        let seed_btn = button(
            row![
                seed_swatch,
                text(format!(
                    "Seed: #{:06X}",
                    color_to_argb(self.seed_color) & 0x00FF_FFFF
                ))
                .size(13),
            ]
            .spacing(10)
            .align_y(Center),
        )
        .on_press(Message::OpenPicker)
        .style(m3_button::outlined);

        let picker: Element<'_, Message> = color_picker_two(self.picker_open, self.seed_color)
            .on_change(Message::SeedColorPicked)
            .on_close(|| Message::ClosePicker)
            .style(m3_picker)
            .into();

        let body = column![
            title,
            row![mode_btn, seed_btn, variant_row]
                .spacing(24)
                .align_y(Center),
            rule::horizontal(1).style(m3_rule::divider),
            self.color_roles_section(),
            rule::horizontal(1).style(m3_rule::divider),
            self.buttons_section(),
            rule::horizontal(1).style(m3_rule::divider),
            self.inputs_section(),
            rule::horizontal(1).style(m3_rule::divider),
            self.cards_section(),
        ]
        .spacing(20)
        .padding(24);

        let scrollable_body = scrollable(body).style(m3_scrollable::default);

        stack![scrollable_body, picker].into()
    }

    fn color_roles_section(&self) -> Element<'_, Message> {
        let s = MaterialTheme::with_variant(
            color_to_argb(self.seed_color),
            self.mode.into_mode(),
            self.variant.into_variant(),
        )
        .scheme;

        column![
            text("All 49 M3 color roles").size(20),
            text("Primary").size(13),
            row![
                swatch("primary", s.primary, s.on_primary),
                swatch("on_primary", s.on_primary, s.primary),
                swatch("primary_container", s.primary_container, s.on_primary_container),
                swatch("on_primary_container", s.on_primary_container, s.primary_container),
                swatch("inverse_primary", s.inverse_primary, s.primary),
            ]
            .spacing(8),
            row![
                swatch("primary_fixed", s.primary_fixed, s.on_primary_fixed),
                swatch("primary_fixed_dim", s.primary_fixed_dim, s.on_primary_fixed),
                swatch("on_primary_fixed", s.on_primary_fixed, s.primary_fixed),
                swatch("on_primary_fixed_variant", s.on_primary_fixed_variant, s.primary_fixed),
            ]
            .spacing(8),
            text("Secondary").size(13),
            row![
                swatch("secondary", s.secondary, s.on_secondary),
                swatch("on_secondary", s.on_secondary, s.secondary),
                swatch("secondary_container", s.secondary_container, s.on_secondary_container),
                swatch("on_secondary_container", s.on_secondary_container, s.secondary_container),
            ]
            .spacing(8),
            row![
                swatch("secondary_fixed", s.secondary_fixed, s.on_secondary_fixed),
                swatch("secondary_fixed_dim", s.secondary_fixed_dim, s.on_secondary_fixed),
                swatch("on_secondary_fixed", s.on_secondary_fixed, s.secondary_fixed),
                swatch("on_secondary_fixed_variant", s.on_secondary_fixed_variant, s.secondary_fixed),
            ]
            .spacing(8),
            text("Tertiary").size(13),
            row![
                swatch("tertiary", s.tertiary, s.on_tertiary),
                swatch("on_tertiary", s.on_tertiary, s.tertiary),
                swatch("tertiary_container", s.tertiary_container, s.on_tertiary_container),
                swatch("on_tertiary_container", s.on_tertiary_container, s.tertiary_container),
            ]
            .spacing(8),
            row![
                swatch("tertiary_fixed", s.tertiary_fixed, s.on_tertiary_fixed),
                swatch("tertiary_fixed_dim", s.tertiary_fixed_dim, s.on_tertiary_fixed),
                swatch("on_tertiary_fixed", s.on_tertiary_fixed, s.tertiary_fixed),
                swatch("on_tertiary_fixed_variant", s.on_tertiary_fixed_variant, s.tertiary_fixed),
            ]
            .spacing(8),
            text("Error").size(13),
            row![
                swatch("error", s.error, s.on_error),
                swatch("on_error", s.on_error, s.error),
                swatch("error_container", s.error_container, s.on_error_container),
                swatch("on_error_container", s.on_error_container, s.error_container),
            ]
            .spacing(8),
            text("Background & Surface").size(13),
            row![
                swatch("background", s.background, s.on_background),
                swatch("on_background", s.on_background, s.background),
                swatch("surface", s.surface, s.on_surface),
                swatch("on_surface", s.on_surface, s.surface),
                swatch("surface_variant", s.surface_variant, s.on_surface_variant),
                swatch("on_surface_variant", s.on_surface_variant, s.surface_variant),
            ]
            .spacing(8),
            text("Surface tones").size(13),
            row![
                swatch("surface_dim", s.surface_dim, s.on_surface),
                swatch("surface_bright", s.surface_bright, s.on_surface),
                swatch("surface_tint", s.surface_tint, s.on_primary),
            ]
            .spacing(8),
            text("Surface containers (elevation)").size(13),
            row![
                swatch("sc_lowest", s.surface_container_lowest, s.on_surface),
                swatch("sc_low", s.surface_container_low, s.on_surface),
                swatch("sc", s.surface_container, s.on_surface),
                swatch("sc_high", s.surface_container_high, s.on_surface),
                swatch("sc_highest", s.surface_container_highest, s.on_surface),
            ]
            .spacing(8),
            text("Outline & inverse").size(13),
            row![
                swatch("outline", s.outline, s.surface),
                swatch("outline_variant", s.outline_variant, s.on_surface),
                swatch("inverse_surface", s.inverse_surface, s.inverse_on_surface),
                swatch("inverse_on_surface", s.inverse_on_surface, s.inverse_surface),
            ]
            .spacing(8),
            text("Shadow & scrim").size(13),
            row![
                swatch("shadow", s.shadow, Color::WHITE),
                swatch("scrim", s.scrim, Color::WHITE),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .into()
    }

    fn buttons_section(&self) -> Element<'_, Message> {
        column![
            text("Buttons").size(20),
            row![
                button(text("Filled")).on_press(Message::ToggleMode).style(m3_button::filled),
                button(text("Tonal")).on_press(Message::ToggleMode).style(m3_button::tonal),
                button(text("Outlined")).on_press(Message::ToggleMode).style(m3_button::outlined),
                button(text("Elevated")).on_press(Message::ToggleMode).style(m3_button::elevated),
                button(text("Text")).on_press(Message::ToggleMode).style(m3_button::text),
            ]
            .spacing(12)
            .align_y(Center),
            text("Disabled").size(13),
            row![
                button(text("Filled")).style(m3_button::filled),
                button(text("Tonal")).style(m3_button::tonal),
                button(text("Outlined")).style(m3_button::outlined),
                button(text("Elevated")).style(m3_button::elevated),
                button(text("Text")).style(m3_button::text),
            ]
            .spacing(12)
            .align_y(Center),
        ]
        .spacing(12)
        .into()
    }

    fn inputs_section(&self) -> Element<'_, Message> {
        column![
            text("Inputs & selection").size(20),
            row![
                column![
                    text("Text input — filled").size(13),
                    text_input("Placeholder", &self.input_text)
                        .on_input(Message::InputChanged)
                        .style(m3_text_input::filled)
                        .padding(12),
                    Space::new().height(Length::Fixed(8.0)),
                    text("Text input — outlined").size(13),
                    text_input("Placeholder", &self.input_text)
                        .on_input(Message::InputChanged)
                        .style(m3_text_input::outlined)
                        .padding(12),
                ]
                .spacing(6)
                .width(Length::Fixed(280.0)),
                column![
                    text("Checkbox / Radio").size(13),
                    checkbox(self.checkbox_a)
                        .label("Option A")
                        .on_toggle(Message::CheckboxA)
                        .style(m3_checkbox::primary),
                    checkbox(self.checkbox_b)
                        .label("Option B")
                        .on_toggle(Message::CheckboxB)
                        .style(m3_checkbox::primary),
                    radio("Alpha", RadioChoice::A, self.radio_choice, Message::RadioSelected)
                        .style(m3_radio::primary),
                    radio("Beta", RadioChoice::B, self.radio_choice, Message::RadioSelected)
                        .style(m3_radio::primary),
                    radio("Gamma", RadioChoice::C, self.radio_choice, Message::RadioSelected)
                        .style(m3_radio::primary),
                ]
                .spacing(6),
                column![
                    text("Switch / Pick list").size(13),
                    toggler(self.toggler_a)
                        .label("Notifications")
                        .on_toggle(Message::TogglerA)
                        .style(m3_toggler::switch),
                    toggler(self.toggler_b)
                        .label("Sync over Wi-Fi only")
                        .on_toggle(Message::TogglerB)
                        .style(m3_toggler::switch),
                    pick_list(
                        self.pick_value,
                        &[PickChoice::One, PickChoice::Two, PickChoice::Three][..],
                        PickChoice::to_string,
                    )
                    .on_select(Message::PickSelected)
                    .placeholder("Choose an option")
                    .style(m3_pick_list::filled)
                    .menu_style(m3_menu::default),
                ]
                .spacing(8),
            ]
            .spacing(24),
            text("Slider & progress").size(13),
            slider(0.0..=100.0, self.slider_value, Message::SliderChanged).style(m3_slider::primary),
            progress_bar(0.0..=100.0, self.slider_value).style(m3_progress_bar::primary),
        ]
        .spacing(12)
        .into()
    }

    fn cards_section(&self) -> Element<'_, Message> {
        let make_card = |kind: &'static str, style: fn(&Theme) -> iced::widget::container::Style| {
            container(
                column![text(kind).size(16), text("This is a sample card body.").size(13)]
                    .spacing(8),
            )
            .style(style)
            .padding(16)
            .width(Length::Fixed(220.0))
        };
        column![
            text("Cards (container styles)").size(20),
            row![
                make_card("Filled card", m3_container::filled_card),
                make_card("Elevated card", m3_container::elevated_card),
                make_card("Outlined card", m3_container::outlined_card),
            ]
            .spacing(16),
        ]
        .spacing(12)
        .into()
    }

    fn theme(&self) -> Theme {
        MaterialTheme::with_variant(
            color_to_argb(self.seed_color),
            self.mode.into_mode(),
            self.variant.into_variant(),
        )
        .into()
    }
}

fn variant_chip<'a>(current: VariantChoice, variant: VariantChoice) -> Element<'a, Message> {
    let label = text(variant.label()).size(13);
    let b = button(label).on_press(Message::SelectVariant(variant));
    if current == variant {
        b.style(m3_button::filled).into()
    } else {
        b.style(m3_button::outlined).into()
    }
}

fn swatch<'a>(label: &'a str, color: Color, on_color: Color) -> Element<'a, Message> {
    container(
        column![
            text(label).size(11).color(on_color),
            text(format!("#{:02X}{:02X}{:02X}", to_u8(color.r), to_u8(color.g), to_u8(color.b)))
                .size(10)
                .color(on_color),
        ]
        .spacing(2),
    )
    .style(m3_container::swatch_box(color, on_color))
    .padding(8)
    .width(Length::Fixed(140.0))
    .height(Length::Fixed(54.0))
    .into()
}

fn to_u8(c: f32) -> u8 {
    (c.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Material Theme Demo")
        .theme(Demo::theme)
        .run()
}
