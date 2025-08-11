use dirs::home_dir;
use iced::Alignment::{self};
use iced::widget::image::Handle;
use iced::widget::{Space, button, column, container, image, progress_bar, row, text};
use iced::{Element, Length, Subscription, Task, Theme, time, window};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

////// Contants ///////////////
const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 600.0;
const DEFAULT_HEADING: &str = "Welcome to your Linux Laptop";
const DEFAULT_BODY: &str = "Thanks for choosing a Linux laptop! \
    \n We want you to have a few applications and settings configured before you get started with your new laptop. \
    \n \nThis process should take 20 to 30 minutes to complete. If you need additional software or help, please contact your Sys admin team.";

const DEFULT_TASK: &str = "Starting configuration...";
//////////////////////////////

#[derive(Debug, Deserialize)]
struct TaskLogLine {
    task_msg: Option<String>,
    task_number: Option<u16>,
    task_complete: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PrepTalk {
    body: String,
    file: Option<PathBuf>,
    heading: String,
    logo: Option<String>,
    task: Option<String>,
    task_number: Option<u16>,
    task_total: Option<u16>,
    completed: Option<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Monitor,
    ExitPressed,
}

impl PrepTalk {
    fn new() -> (Self, Task<Message>) {
        let config_contents = Self::load_config_or_default();
        (Self { ..config_contents }, Task::none())
    }

    fn load_config_or_default() -> PrepTalk {
        //TODO: remove the expect
        let home_path: PathBuf = home_dir().expect("No home directory found");
        let config_path = home_path.join(".prepTalk.toml");

        let default_config = PrepTalk {
            logo: None,
            heading: String::from(DEFAULT_HEADING),
            body: String::from(DEFAULT_BODY),
            //TODO remove unwrap
            file: Some(PathBuf::from_str("./prep.log").unwrap()),
            task: Some(String::from(DEFULT_TASK)),
            task_total: None,
            task_number: None,
            completed: Some(false),
        };

        let config_contents: PrepTalk = match fs::read_to_string(config_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(conf) => conf,
                Err(toml_error) => {
                    eprint!("Can't parse toml config: {toml_error}");
                    default_config
                }
            },
            Err(fs_error) => {
                eprint!("Can't parse toml config: {fs_error}");
                default_config
            }
        };
        config_contents
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_secs(3)).map(|_| Message::Monitor)
    }

    fn update(state: &mut PrepTalk, message: Message) {
        match message {
            Message::Monitor => {
                if let Some(path) = &state.file {
                    if let Some(log_line) = read_last_line(path) {
                        if log_line.task_msg.is_some() {
                            state.task = log_line.task_msg;
                        }
                        if log_line.task_number.is_some() {
                            state.task_number = log_line.task_number;
                        }
                        if log_line.task_complete.is_some() {
                            println!("here");
                            state.completed = Some(true);
                        }
                    }
                };
            }
            Message::ExitPressed => {
                std::process::exit(0);
            }
        }
    }

    fn view(state: &PrepTalk) -> Element<Message> {
        let logo = match &state.logo {
            Some(path) => row![image(Handle::from_path(path)).width(Length::Fixed(350.0))].padding(
                iced::Padding {
                    top: 8.0,
                    right: 0.0,
                    bottom: 20.0,
                    left: 0.0,
                },
            ),

            None => row![Space::with_height(0)].padding([80, 20]),
        };

        let heading = row![text(&state.heading).size(30)].padding(iced::Padding {
            top: 8.0,
            right: 0.0,
            bottom: 20.0,
            left: 0.0,
        });
        let body = text(&state.body)
            .align_x(Alignment::Center)
            .line_height(1.5);

        let progress = if let (Some(total), Some(number)) = (state.task_total, state.task_number) {
            let percent = (number as f32 / total as f32) * 100.0;
            // println!("Progress: {percent}%");
            percent.clamp(0.0, 100.0)
        } else {
            0.0
        };

        let progress_bar = progress_bar(0.0..=100.0, progress).height(Length::Fixed(10.0));
        let progress_task = text(state.task.as_deref().unwrap_or(DEFULT_TASK)).size(11);

        let progress_area: iced::Element<Message> = if let Some(true) = state.completed {
            button(text("Setup completed"))
                .on_press(Message::ExitPressed)
                .padding(10)
                .into()
        } else {
            column![progress_bar, progress_task]
                .align_x(Alignment::Center)
                .spacing(10)
                .into()
        };

        container(
            column![
                logo,
                heading,
                body,
                Space::with_height(Length::Fill),
                progress_area
            ]
            .align_x(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([20, 40])
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

fn main() -> iced::Result {
    let app_settings = window::Settings {
        size: iced::Size {
            width: (WINDOW_WIDTH),
            height: (WINDOW_HEIGHT),
        },
        resizable: false,
        decorations: true,
        ..Default::default()
    };

    iced::application("PrepTalk", PrepTalk::update, PrepTalk::view)
        .theme(PrepTalk::theme)
        .window(app_settings)
        .subscription(PrepTalk::subscription)
        .run_with(PrepTalk::new)
}

//Spinning bar function
//increment thebar by one percent in a loop

fn read_last_line(path: &PathBuf) -> Option<TaskLogLine> {
    let content = std::fs::read_to_string(path).ok()?;
    let last_line = content.lines().last()?.trim();

    // Examples handled:
    // - task_msg = "Installing" task_number = 4
    // - task_msg="Hello 3" task_number=4
    let normalized = last_line
        .replace(" task_msg", "\ntask_msg")
        .replace("\ttask_msg", "\ntask_msg")
        .replace(" task_number", "\ntask_number")
        .replace("\ttask_number", "\ntask_number")
        .replace(" task_complete", "\ntask_complete")
        .replace("\ttask_complete", "\ntask_complete");

    match toml::from_str::<TaskLogLine>(&normalized) {
        Ok(log) => {
            if log.task_msg.is_none() && log.task_number.is_none() && log.task_complete.is_none() {
                // If the line parsed but contains no useful info for the UI, return None
                None
            } else {
                // Otherwise, return the parsed TaskLogLine
                println!("{:?}", log);
                Some(log)
            }
        }
        Err(_) => None, // If it's not valid TOML, ignore
    }
}
