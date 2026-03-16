use cosmic::app::Core;
use cosmic::iced::{Color, Length, Subscription, Task};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced_runtime::core::window;
use cosmic::{Action, Application, Element};
use std::time::Duration;

const REFRESH_SECS: u64 = 30;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<BatteryApplet>(())
}

struct BatteryApplet {
    core: Core,
    percentage: Option<f64>,
    charging: bool,
    present: bool,
    popup: Option<window::Id>,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    TogglePopup,
    BatteryInfo { percentage: f64, charging: bool, present: bool },
}

fn battery_color(pct: f64, charging: bool) -> Color {
    if charging {
        return Color::from_rgb(0.0, 0.898, 1.0);
    }
    match pct as u32 {
        80..=100 => Color::from_rgb(0.412, 1.0, 0.278),
        40..=79  => Color::WHITE,
        15..=39  => Color::from_rgb(1.0, 0.702, 0.0),
        _        => Color::from_rgb(1.0, 0.090, 0.267),
    }
}

fn battery_status_text(pct: f64, charging: bool) -> &'static str {
    if charging { return "Charging"; }
    match pct as u32 {
        80..=100 => "Good",
        40..=79  => "Normal",
        15..=39  => "Low — consider charging",
        _        => "Critical — plug in now!",
    }
}

impl Application for BatteryApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.github.cosmic-battery-applet";

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, Task<Action<Message>>) {
        let applet = Self {
            core,
            percentage: None,
            charging: false,
            present: false,
            popup: None,
        };
        (applet, cosmic::task::future(async {
            cosmic::Action::App(match fetch_battery().await {
                Some((pct, charging, present)) => Message::BatteryInfo { percentage: pct, charging, present },
                None => Message::Tick,
            })
        }))
    }

    fn update(&mut self, msg: Message) -> Task<Action<Message>> {
        match msg {
            Message::Tick => cosmic::task::future(async {
                cosmic::Action::App(match fetch_battery().await {
                    Some((pct, charging, present)) => Message::BatteryInfo { percentage: pct, charging, present },
                    None => Message::Tick,
                })
            }),
            Message::BatteryInfo { percentage, charging, present } => {
                self.percentage = Some(percentage);
                self.charging = charging;
                self.present = present;
                Task::none()
            }
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(p)
                } else {
                    let new_id = window::Id::unique();
                    self.popup = Some(new_id);
                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        Some((220, 140)),
                        None,
                        None,
                    );
                    cosmic::iced::platform_specific::shell::commands::popup::get_popup(popup_settings)
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(REFRESH_SECS)).map(|_| Message::Tick)
    }

    // Main panel button
    fn view(&self) -> Element<Message> {
        let (label, color) = match (self.present, self.percentage) {
            (false, _) | (_, None) => ("--".to_string(), Color::WHITE),
            (true, Some(pct)) => {
                let label = if self.charging {
                    if pct >= 100.0 { format!("+{:.0}", pct) } else { format!("+{:.0}%", pct) }
                } else {
                    format!("{:.0}%", pct)
                };
                (label, battery_color(pct, self.charging))
            }
        };

        cosmic::iced::widget::container(
            cosmic::widget::button::custom(
                cosmic::widget::text(label).class(cosmic::theme::Text::Color(color))
            )
            .on_press(Message::TogglePopup)
            .class(cosmic::theme::Button::Text)
        )
        .width(Length::Shrink)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    // Popup window
    fn view_window(&self, id: window::Id) -> Element<Message> {
        if self.popup != Some(id) {
            return cosmic::widget::text("").into();
        }

        let (pct, color, status) = match (self.present, self.percentage) {
            (false, _) | (_, None) => (
                "--".to_string(),
                Color::WHITE,
                "No battery detected",
            ),
            (true, Some(pct)) => (
                format!("{:.0}%", pct),
                battery_color(pct, self.charging),
                battery_status_text(pct, self.charging),
            ),
        };

        let charging_line = if self.charging {
            "⚡ Currently charging"
        } else {
            "🔋 On battery"
        };

        let content = cosmic::widget::column::with_children(vec![
            cosmic::widget::text("Battery")
                .size(16)
                .into(),
            cosmic::widget::divider::horizontal::default().into(),
            cosmic::widget::text(format!("Charge: {}", pct))
                .class(cosmic::theme::Text::Color(color))
                .size(15)
                .into(),
            cosmic::widget::text(charging_line)
                .size(13)
                .into(),
            cosmic::widget::text(status)
                .size(13)
                .into(),
        ])
        .spacing(8)
        .padding(16);

        self.core.applet.popup_container(content).into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

async fn fetch_battery() -> Option<(f64, bool, bool)> {
    use zbus::Connection;
    let conn = Connection::system().await.ok()?;

    let paths: Vec<zbus::zvariant::OwnedObjectPath> = conn
        .call_method(
            Some("org.freedesktop.UPower"),
            "/org/freedesktop/UPower",
            Some("org.freedesktop.UPower"),
            "EnumerateDevices",
            &(),
        )
        .await.ok()?.body().deserialize().ok()?;

    for path in paths {
        let dev_type: u32 = conn
            .call_method(Some("org.freedesktop.UPower"), path.as_str(),
                Some("org.freedesktop.DBus.Properties"), "Get",
                &("org.freedesktop.UPower.Device", "Type"))
            .await.ok()?.body().deserialize::<zbus::zvariant::Value>().ok()
            .and_then(|v| v.downcast::<u32>().ok())?;

        if dev_type != 2 { continue; }

        let percentage: f64 = conn
            .call_method(Some("org.freedesktop.UPower"), path.as_str(),
                Some("org.freedesktop.DBus.Properties"), "Get",
                &("org.freedesktop.UPower.Device", "Percentage"))
            .await.ok()?.body().deserialize::<zbus::zvariant::Value>().ok()
            .and_then(|v| v.downcast::<f64>().ok())?;

        let state: u32 = conn
            .call_method(Some("org.freedesktop.UPower"), path.as_str(),
                Some("org.freedesktop.DBus.Properties"), "Get",
                &("org.freedesktop.UPower.Device", "State"))
            .await.ok()?.body().deserialize::<zbus::zvariant::Value>().ok()
            .and_then(|v| v.downcast::<u32>().ok())?;

        return Some((percentage, state == 1 || state == 4, true));
    }

    Some((0.0, false, false))
}