use iced::{executor, Application, Command, Element, Settings};
use iced::widget::{Row, button, Button, text_input, TextInput, Radio, Text, Column, Container};
use iced::Length::FillPortion;
use iced::alignment::Horizontal;

fn main() -> iced::Result {
    let mut gm_settings = Settings::default();
    gm_settings.id = Some(String::from("grademapper-rs"));
    // Font fallback'a в iced нет. Страдайте.
    gm_settings.default_font = Some(include_bytes!("/usr/share/fonts/TTF/OpenSans-Regular.ttf")); 
    GradeMapper::run(gm_settings)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkType {
    Classwork,
    Test1,
    Test2,
    VocalExam,
    Exam
}

#[derive(Debug, Clone)]
enum Message {
    EditGrade(String),
    WorkTypeChoice(WorkType),
    ProcessGrade,
    RemoveGrade,
}

struct GradeMapper {
    text_inp_state: text_input::State,
    remove_button_state: button::State,
    submit_button_state: button::State,
    work_type_selected: Option<WorkType>, 
    current_grade: String,
    current_weight: f32,
    grades: Vec<i32>,
    weights: Vec<f32>,
    avg: String,
}

impl Application for GradeMapper {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (GradeMapper{
            text_inp_state: text_input::State::new(),
            remove_button_state: button::State::new(),
            submit_button_state: button::State::new(),
            work_type_selected: Some(WorkType::Classwork),
            current_grade: String::new(),
            current_weight: 1.0,
            grades: Vec::new(), 
            weights: Vec::new(), 
            avg: String::new()
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("GradeMapper (Rust edition)")
    }

    fn update(&mut self, click: Self::Message) -> Command<Self::Message> {
        match click {
            Message::ProcessGrade => {
                if let Ok(res) = self.current_grade.parse() {
                    if res > 5 || res < 1 { // проверка на валидность оценки
                        self.avg = String::from("ОШИБКА: неверная оценка");
                    } else {
                        self.grades.push(res);
                        self.weights.push(self.current_weight);
                        self.compute_avg();
                    }
                    self.work_type_selected = Some(WorkType::Classwork);
                    self.current_weight = 1.0;
                } else {
                    self.avg = String::from("ОШИБКА: не является числом"); 
                }
                self.current_grade = String::new();
            },
            Message::WorkTypeChoice(t) => {
                match t {
                    WorkType::Classwork => self.current_weight = 1.0,
                    WorkType::Test1 => self.current_weight = 1.2,
                    WorkType::Test2 => self.current_weight = 1.3,
                    WorkType::VocalExam => self.current_weight = 1.4,
                    WorkType::Exam => self.current_weight = 1.5,
                };
                self.work_type_selected = Some(t);
            },
            Message::RemoveGrade => {
                self.weights.remove(self.weights.len()-1);
                self.grades.remove(self.grades.len()-1);
                self.compute_avg(); // пересчитываем
            },
            Message::EditGrade(s) => self.current_grade = s.clone(),
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let avg_out = Text::new(format!("Средний балл: {}", self.avg))
            .horizontal_alignment(Horizontal::Center);

        // Вероятно, кнопки центрируются через .style(), но разбираться в нём мне сейчас лень
        let submit_button = Button::new(&mut self.submit_button_state, 
                                        Text::new("Рассчитать").horizontal_alignment(Horizontal::Center))
            .on_press(Message::ProcessGrade).width(FillPortion(1)).padding(10);
        let rm_button = Button::new(&mut self.remove_button_state, 
                                    Text::new("Удалить прошлую оценку").horizontal_alignment(Horizontal::Center))
            .on_press(Message::RemoveGrade).width(FillPortion(1)).padding(10);
        let grade_in = TextInput::new(&mut self.text_inp_state, "Введите оценку", &self.current_grade, Message::EditGrade)
            .padding(10);

        let type_classwork = Radio::new(WorkType::Classwork, "Классная работа", self.work_type_selected, Message::WorkTypeChoice);
        let type_test1 = Radio::new(WorkType::Test1, "Самостоятельная работа", self.work_type_selected, Message::WorkTypeChoice);
        let type_test2 = Radio::new(WorkType::Test2, "Проверочная работа", self.work_type_selected, Message::WorkTypeChoice);
        let type_vocal_exam = Radio::new(WorkType::VocalExam, "Зачёт", self.work_type_selected, Message::WorkTypeChoice);
        let type_exam = Radio::new(WorkType::Exam, "Контрольная работа", self.work_type_selected, Message::WorkTypeChoice);

        let layout = Row::new()
            .push(Column::new()
                  .push(grade_in).push(avg_out)
                  .spacing(10).padding(10).width(FillPortion(3)))
            .push(Column::new()
                  .push(type_classwork).push(type_test1)
                  .push(type_test2).push(type_vocal_exam)
                  .push(type_exam)
                  .spacing(10).padding(10).width(FillPortion(5)))
            .push(Column::new()
                  .push(submit_button).push(rm_button)
                  .spacing(10).padding(10).width(FillPortion(5)))
            .spacing(10).padding(20);

        let container = Container::new(layout)
            .center_x().center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill).into();
        
        return container;
    }
}
impl GradeMapper {
    // находим среднее взвешенное
    fn compute_avg(&mut self) {
        let mut sum_grades = 0.0;
        let mut sum_weights = 0.0;
        for i in 0..self.grades.len() {
            sum_grades += self.grades[i] as f32 * self.weights[i];
            sum_weights += self.weights[i];
        }
        let average = sum_grades as f32 / sum_weights;
        self.avg = average.to_string();
    }
}
