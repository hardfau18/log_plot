use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use std::cell::Cell;
use std::convert::TryFrom;
use std::error::Error;
use std::f64;

use plotters::prelude::*;
use plotters_cairo::CairoBackend;

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::GaussianPlot)]
pub struct GaussianPlot {
    #[property(get, set, minimum = -f64::consts::PI, maximum = f64::consts::PI)]
    pitch: Cell<f64>,
    #[property(get, set, minimum = 0.0, maximum = f64::consts::PI)]
    yaw: Cell<f64>,
    #[property(get, set, minimum = -10.0, maximum = 10.0)]
    mean_x: Cell<f64>,
    #[property(get, set, minimum = -10.0, maximum = 10.0)]
    mean_y: Cell<f64>,
    #[property(get, set, minimum = 0.0, maximum = 10.0)]
    std_x: Cell<f64>,
    #[property(get, set, minimum = 0.0, maximum = 10.0)]
    std_y: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for GaussianPlot {
    const NAME: &'static str = "GaussianPlot";
    type Type = super::GaussianPlot;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for GaussianPlot {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        Self::derived_set_property(self, id, value, pspec);
        self.obj().queue_draw();
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        Self::derived_property(self, id, pspec)
    }
}

impl WidgetImpl for GaussianPlot {
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let width = self.obj().width() as u32;
        let height = self.obj().height() as u32;
        if width == 0 || height == 0 {
            return;
        }

        let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
        let cr = snapshot.append_cairo(&bounds);
        let backend = CairoBackend::new(&cr, (width, height)).unwrap();
        self.plot_matshow(backend).unwrap();
    }
}

/// (core_id, [(taskid, start us, end us)])
type Logs = [(u8, &'static [(u8, u8, u8)]); 10];

const LOGS: Logs = [
    (
        0,
        &[
            (4, 4, 6),
            (6, 6, 10),
            (8, 10, 11),
            (6, 11, 17),
            (8, 17, 34),
            (4, 34, 39),
            (0, 39, 41),
            (3, 41, 68),
            (1, 68, 71),
            (8, 71, 73),
            (10, 73, 80),
            (8, 80, 84),
            (12, 84, 86),
            (4, 86, 90),
            (1, 90, 95),
            (12, 95, 97),
            (0, 97, 99),
        ],
    ),
    (
        1,
        &[
            (13, 0, 17),
            (1, 17, 19),
            (7, 19, 26),
            (5, 26, 27),
            (10, 27, 37),
            (4, 37, 39),
            (15, 39, 43),
            (0, 43, 56),
            (0, 56, 65),
            (9, 65, 66),
            (13, 66, 76),
            (13, 76, 83),
            (15, 83, 85),
            (3, 85, 86),
            (0, 86, 87),
            (9, 87, 91),
            (6, 91, 95),
            (2, 95, 100),
        ],
    ),
    (
        2,
        &[
            (15, 1, 3),
            (14, 3, 16),
            (8, 16, 19),
            (9, 19, 26),
            (5, 26, 27),
            (4, 27, 31),
            (15, 31, 33),
            (14, 33, 41),
            (7, 41, 53),
            (2, 53, 57),
            (11, 57, 58),
            (8, 58, 64),
            (2, 64, 67),
            (4, 67, 69),
            (8, 69, 70),
            (7, 70, 74),
            (12, 74, 82),
            (2, 82, 93),
            (9, 93, 97),
            (9, 97, 99),
        ],
    ),
    (
        3,
        &[
            (2, 2, 4),
            (7, 4, 6),
            (0, 6, 8),
            (10, 8, 11),
            (10, 11, 12),
            (6, 12, 21),
            (15, 21, 26),
            (0, 26, 36),
            (8, 36, 49),
            (0, 49, 68),
            (14, 68, 69),
            (9, 69, 71),
            (3, 71, 79),
            (8, 79, 81),
            (13, 81, 85),
            (10, 85, 88),
            (10, 88, 91),
            (13, 91, 99),
            (15, 99, 100),
        ],
    ),
    (
        4,
        &[
            (7, 7, 29),
            (8, 29, 31),
            (1, 31, 41),
            (6, 41, 47),
            (1, 47, 49),
            (8, 49, 50),
            (7, 50, 53),
            (13, 53, 58),
            (11, 58, 59),
            (0, 59, 63),
            (12, 63, 64),
            (12, 64, 66),
            (3, 66, 70),
            (4, 70, 74),
            (2, 74, 75),
            (1, 75, 86),
            (13, 86, 91),
            (12, 91, 92),
        ],
    ),
    (
        5,
        &[
            (8, 1, 2),
            (14, 2, 3),
            (4, 3, 12),
            (10, 12, 13),
            (5, 13, 18),
            (3, 18, 22),
            (14, 22, 30),
            (2, 30, 37),
            (4, 37, 47),
            (2, 47, 56),
            (10, 56, 57),
            (15, 57, 59),
            (8, 59, 64),
            (13, 64, 76),
            (2, 76, 78),
            (0, 78, 80),
            (15, 80, 84),
            (4, 84, 94),
            (3, 94, 99),
        ],
    ),
    (
        6,
        &[
            (14, 1, 4),
            (8, 4, 6),
            (2, 6, 10),
            (7, 10, 17),
            (4, 17, 25),
            (12, 25, 27),
            (13, 27, 28),
            (7, 28, 29),
            (0, 29, 47),
            (5, 47, 48),
            (7, 48, 52),
            (8, 52, 55),
            (6, 55, 64),
            (7, 64, 82),
            (4, 82, 89),
            (3, 89, 91),
            (7, 91, 100),
        ],
    ),
    (
        7,
        &[
            (0, 4, 6),
            (2, 6, 10),
            (1, 10, 25),
            (10, 25, 29),
            (3, 29, 30),
            (5, 30, 31),
            (0, 31, 37),
            (10, 37, 50),
            (14, 50, 52),
            (10, 52, 57),
            (7, 57, 62),
            (5, 62, 65),
            (8, 65, 66),
            (12, 66, 76),
            (10, 76, 78),
            (1, 78, 79),
            (0, 79, 81),
            (12, 81, 88),
            (2, 88, 91),
        ],
    ),
    (
        8,
        &[
            (8, 10, 15),
            (11, 15, 18),
            (5, 18, 21),
            (12, 21, 25),
            (15, 25, 28),
            (2, 28, 30),
            (1, 30, 34),
            (2, 34, 35),
            (8, 35, 43),
            (13, 43, 45),
            (0, 45, 59),
            (6, 59, 62),
            (10, 62, 72),
            (10, 72, 74),
            (11, 74, 77),
            (2, 77, 88),
            (7, 88, 89),
            (3, 89, 95),
            (5, 95, 98),
            (9, 98, 99),
        ],
    ),
    (
        9,
        &[
            (14, 0, 2),
            (13, 2, 3),
            (11, 3, 6),
            (12, 6, 7),
            (14, 7, 15),
            (15, 15, 33),
            (7, 33, 38),
            (8, 38, 41),
            (12, 41, 46),
            (11, 46, 50),
            (12, 50, 57),
            (15, 57, 59),
            (1, 59, 64),
            (0, 64, 67),
            (15, 67, 70),
            (9, 70, 74),
            (13, 74, 81),
            (12, 81, 91),
            (2, 91, 92),
            (2, 92, 99),
        ],
    ),
];

struct Task {
    core_id: u32,
    task_id: u32,
    start: u32,
    duration: u32,
}

impl GaussianPlot {
    fn plot_matshow<'a, DB: DrawingBackend + 'a>(
        &self,
        backend: DB,
    ) -> Result<(), Box<dyn Error + 'a>> {
        let root = backend.into_drawing_area();

        root.fill(&WHITE)?;
        let cores: Vec<_> = LOGS.iter().map(|rec| rec.0).collect();
        let mut min_timings = u32::MAX;
        let mut max_timings = u32::MIN;
        let mut tasks = Vec::new();
        LOGS.iter().for_each(|(core_id, core_logs)| {
            core_logs.iter().for_each(|log| {
                let task = Task {
                    core_id: *core_id as u32,
                    task_id: log.0 as u32,
                    start: log.1 as u32,
                    duration: (log.2.checked_sub(log.1).unwrap()) as u32,
                };
                if task.start < min_timings {
                    min_timings = task.start;
                }
                if task.start + task.duration > max_timings {
                    max_timings = task.start + task.duration
                }
                tasks.push(task);
            });
        });

        let min_core = *cores.iter().min().unwrap() as u32;
        let max_core = *cores.iter().max().unwrap() as u32;
        let mut chart = ChartBuilder::on(&root)
            .caption("Log plot", ("sans-serif", 40))
            .margin(5)
            .set_left_and_bottom_label_area_size(40)
            .build_cartesian_2d(min_timings..max_timings, min_core..max_core)?;

        chart
            .configure_mesh()
            .max_light_lines(4)
            // .disable_x_mesh()
            // .disable_y_mesh()
            .label_style(("sans-serif", 15))
            .draw()?;

        let style = ShapeStyle {
            color: BLUE.mix(0.6),
            filled: true,
            stroke_width: 0,
        };
        let series = tasks.iter().map(|task| {
            plotters::element::Rectangle::new(
                [
                    (task.start, task.core_id + 1),
                    (task.start + task.duration, task.core_id),
                ],
                style,
            )
        });
        chart.draw_series(series)?;
        root.present()?;
        Ok(())
    }
}
