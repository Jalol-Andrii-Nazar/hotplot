pub mod data;

use iced::{Point, Rectangle, Size, Vector};
use iced::canvas::{Cache, Cursor, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{HorizontalAlignment, VerticalAlignment};

use self::data::{AxisData, AxisValue, PlotSettings, Settings};

pub struct ChartBuilder<
    XV: AxisValue,
    YV: AxisValue,
    XD: AxisData<XV>,
    YD: AxisData<YV>,
> {
    settings: Settings,
    min_x_value_opt: Option<XV>,
    max_x_value_opt: Option<XV>,
    min_y_value_opt: Option<YV>,
    max_y_value_opt: Option<YV>,
    data: Vec<(PlotSettings, Vec<(XD, YD)>)>,
}

impl <XV: AxisValue, YV: AxisValue, XD: AxisData<XV>, YD: AxisData<YV>> ChartBuilder<XV, YV, XD, YD> {
    pub fn new(settings: data::Settings) -> Self {
        Self {
            settings,
            min_x_value_opt: None,
            max_x_value_opt: None,
            min_y_value_opt: None,
            max_y_value_opt: None,
            data: Vec::new(),
        }
    }

    pub fn build(self) -> Chart<XV, YV, XD, YD> {
        assert!(self.min_x_value_opt.is_some(), "There is no min_x_value!");
        assert!(self.max_x_value_opt.is_some(), "There is no max_x_value!");
        assert!(self.min_y_value_opt.is_some(), "There is no min_y_value!");
        assert!(self.max_y_value_opt.is_some(), "There is no max_y_value!");
        let settings = self.settings;
        let min_x_value = self.min_x_value_opt.unwrap();
        let max_x_value = self.max_x_value_opt.unwrap();
        let min_y_value = self.min_y_value_opt.unwrap();
        let max_y_value = self.max_y_value_opt.unwrap();
        let data = self.data;
        Chart::new(
            settings,
            min_x_value,
            max_x_value,
            min_y_value,
            max_y_value,
            data,
        )
    }

    pub fn data(mut self, data: Vec<(PlotSettings, Vec<(XD, YD)>)>) -> Self {
        self.data = data;
        self
    }

    pub fn add_data(mut self, plot_settings: PlotSettings, edges: Vec<(XD, YD)>) -> Self {
        self.data.push((plot_settings, edges));
        self
    }

    pub fn min_x_value(mut self, min_x_value: XV) -> Self {
        self.min_x_value_opt = Some(min_x_value);
        self
    }

    pub fn max_x_value(mut self, max_x_value: XV) -> Self {
        self.max_x_value_opt = Some(max_x_value);
        self
    }

    pub fn min_y_value(mut self, min_y_value: YV) -> Self {
        self.min_y_value_opt = Some(min_y_value);
        self
    }

    pub fn max_y_value(mut self, max_y_value: YV) -> Self {
        self.max_y_value_opt = Some(max_y_value);
        self
    }

    pub fn calculate_min_x_value(mut self) -> Self {
        assert!(self.data.iter().any(|(_settings, vec)| !vec.is_empty()));
        let min_x_value = self
            .data
            .iter()
            .map(|(_settings, vec)| vec)
            .flat_map(|vec| vec.iter().map(|(xv, _yv)| xv.value()))
            .min_by(|xv1, xv2| xv1.compare_value(xv2))
            .unwrap()
            .clone();
        self.min_x_value_opt = Some(min_x_value);
        self
    }

    pub fn calculate_max_x_value(mut self) -> Self {
        assert!(self.data.iter().any(|(_settings, vec)| !vec.is_empty()));
        let max_x_value = self
            .data
            .iter()
            .map(|(_settings, vec)| vec)
            .flat_map(|vec| vec.iter().map(|(xv, _yv)| xv.value()))
            .max_by(|xv1, xv2| xv1.compare_value(xv2))
            .unwrap()
            .clone();
        self.max_x_value_opt = Some(max_x_value);
        self
    }

    pub fn calculate_min_y_value(mut self) -> Self {
        assert!(self.data.iter().any(|(_settings, vec)| !vec.is_empty()));
        let min_y_value = self
            .data
            .iter()
            .map(|(_settings, vec)| vec)
            .flat_map(|vec| vec.iter().map(|(_xv, yv)| yv.value()))
            .min_by(|yv1, yv2| yv1.compare_value(yv2))
            .unwrap()
            .clone();
        self.min_y_value_opt = Some(min_y_value);
        self
    }

    pub fn calculate_max_y_value(mut self) -> Self {
        assert!(self.data.iter().any(|(_settings, vec)| !vec.is_empty()));
        let max_y_value = self
            .data
            .iter()
            .map(|(_settings, vec)| vec)
            .flat_map(|vec| vec.iter().map(|(_xv, yv)| yv.value()))
            .max_by(|yv1, yv2| yv1.compare_value(yv2))
            .unwrap()
            .clone();
        self.max_y_value_opt = Some(max_y_value);
        self
    }

    pub fn calculate_min_max_x_values(self) -> Self {
        self.calculate_min_x_value()
            .calculate_max_x_value()
    }

    pub fn calculate_min_max_y_values(self) -> Self {
        self.calculate_min_y_value()
            .calculate_max_y_value()
    }

    pub fn calculate_min_max_values(self) -> Self {
        self.calculate_min_max_x_values()
            .calculate_min_max_y_values()
    }
}

pub struct Chart<XV: AxisValue, YV: AxisValue, XD: AxisData<XV>, YD: AxisData<YV>> {
    settings: Settings,
    min_x_value: XV,
    max_x_value: XV,
    total_x_distance: f32,
    min_y_value: YV,
    max_y_value: YV,
    total_y_distance: f32,
    data: Vec<(PlotSettings, Vec<(XD, YD)>)>,
    cache: Cache,
}

impl<XV: AxisValue, YV: AxisValue, XD: AxisData<XV>, YD: AxisData<YV>> Chart<XV, YV, XD, YD> {
    pub fn new(
        settings: data::Settings,
        min_x_value: XV,
        max_x_value: XV,
        min_y_value: YV,
        max_y_value: YV,
        data: Vec<(data::PlotSettings, Vec<(XD, YD)>)>,
    ) -> Self {
        let total_x_distance = min_x_value.distance_to(&max_x_value);
        let total_y_distance = min_y_value.distance_to(&max_y_value);
        Self {
            settings,
            min_x_value,
            max_x_value,
            total_x_distance,
            min_y_value,
            max_y_value,
            total_y_distance,
            data,
            cache: Cache::default(),
        }
    }

    fn points(&self, size: Size) -> Vec<(PlotSettings, Vec<(Point, XD, YD)>)> {
        let width = size.width;
        let height = size.height;
        let result: Vec<(PlotSettings, Vec<(Point, XD, YD)>)> = self
            .data
            .iter()
            .map(|(plot_settings, edges)| {
                let result: Vec<(Point, XD, YD)> = edges
                    .iter()
                    .map(|(x, y)| {
                        let x_distance = self.min_x_value.distance_to(&x.value());
                        let x_coord = crate::math::map_inverval_value(
                            x_distance,
                            (0.0, self.total_x_distance),
                            (0.0, width),
                        );
                        let y_distance = self.min_y_value.distance_to(&y.value());
                        let y_coord = crate::math::map_inverval_value(
                            y_distance,
                            (0.0, self.total_y_distance),
                            (0.0, height),
                        );
                        let point = Point::new(x_coord, height - y_coord);
                        (point, x.to_owned(), y.to_owned())
                    })
                    .collect();
                (plot_settings.clone(), result)
            })
            .collect();
        result
    }

    fn draw_y_label(
        &self,
        frame: &mut Frame,
        padded_area: Rectangle,
        y: f32,
        text: &str,
    ) {
        let theme = self.settings.theme.clone();
        let width = frame.width();
        frame.stroke(
            &Path::line(
                Point::new(padded_area.x - 3.0, y),
                Point::new(padded_area.x + 3.0, y),
            ),
            Stroke {
                color: theme.y_label_short_line_color,
                width: theme.y_label_short_line_width,
                ..Default::default()
            },
        );
        frame.stroke(
            &Path::line(
                Point::new(padded_area.x, y),
                Point::new(width - padded_area.x, y),
            ),
            Stroke {
                color: theme.y_label_long_line_color,
                width: theme.y_label_long_line_width,
                ..Default::default()
            },
        );
        frame.fill_text(Text {
            content: format!("{}", text),
            color: theme.y_label_text_color,
            position: Point::new(padded_area.x - 5.0, y),
            horizontal_alignment: HorizontalAlignment::Right,
            vertical_alignment: VerticalAlignment::Center,
            size: theme.y_label_text_size,
            ..Default::default()
        });
    }

    fn draw_x_label(
        &self,
        frame: &mut Frame,
        padded_area: Rectangle,
        x: f32,
        text: &str
    ) {
        let theme = self.settings.theme.clone();
        let height = frame.height();
        frame.stroke(
            &Path::line(
                Point::new(x, height - padded_area.y - 3.0),
                Point::new(x, height - padded_area.y + 3.0),
            ),
            Stroke {
                color: theme.x_label_short_line_color,
                width: theme.x_label_short_line_width,
                ..Default::default()
            },
        );
        frame.stroke(
            &Path::line(
                Point::new(x, padded_area.y),
                Point::new(x, height - padded_area.y),
            ),
            Stroke {
                color: theme.x_label_long_line_color,
                width: theme.x_label_long_line_width,
                ..Default::default()
            },
        );
        frame.fill_text(Text {
            content: format!("{}", text),
            color: theme.x_label_text_color,
            position: Point::new(x, height - padded_area.y + 5.0),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Top,
            size: theme.x_label_text_size,
            ..Default::default()
        });
    }
}

impl <XV: data::AxisValue, YV: data::AxisValue, XD: data::AxisData<XV>, YD: data::AxisData<YV>> Program<data::Message> for Chart<XV, YV, XD, YD> {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let theme = self.settings.theme.clone();
        
        let size = bounds.size();
        let width = bounds.width;
        let height = bounds.height;

        let (ptop, pright, pbottom, pleft) = self.settings.padding.get(size);
        let (mtop, mright, mbottom, mleft) = self.settings.margin.get(size);

        let full_area = Rectangle::new(Point::ORIGIN, size);
        let padded_area = self.settings.padding.transform(full_area);
        let margined_area = self.settings.margin.transform(padded_area);

        let cursor_position_opt = cursor.position_in(&bounds);
        let padded_cursor_position_opt =
            cursor_position_opt.map(|cp| Point::new(cp.x - padded_area.x, cp.y - padded_area.y));
        let margined_cursor_position_opt = cursor_position_opt
            .map(|cp| Point::new(cp.x - margined_area.x, cp.y - margined_area.y));

        let result = self.cache.draw(size, |frame| {
            frame.fill(
                &Path::rectangle(full_area.position(), full_area.size()),
                self.settings.theme.background_color,
            );
            frame.fill(
                &Path::rectangle(padded_area.position(), padded_area.size()),
                self.settings.theme.padded_background_color,
            );
            self.settings.theme.margined_background_color.iter().for_each(|margined_background_color| {
                frame.fill(
                    &Path::rectangle(margined_area.position(), margined_area.size()),
                    *margined_background_color,
                );
            });

            //Draw name
            self.settings.title
                .as_ref()
                .iter()
                .for_each(|title| {
                    frame.fill_text(Text {
                        content: (*title).clone(),
                        position: Point::new(pleft, ptop / 2.0),
                        color: self.settings.theme.title_color,
                        size: self.settings.theme.title_size,
                        horizontal_alignment: HorizontalAlignment::Left,
                        vertical_alignment: VerticalAlignment::Center,
                        ..Default::default()
                    });
        
                });
            //Draw y labels
            let min_y_label_distance = self.settings.min_y_label_distance.get(margined_area.size());
            let min_y_label_distance_mapped = crate::math::map_inverval_value(
                min_y_label_distance,
                (0.0, margined_area.height),
                (0.0, self.total_y_distance),
            );
            let optimal_y_label_distance =
                margined_area.height / (margined_area.height / min_y_label_distance).floor();
            let optimal_y_label_distance_mapped = crate::math::map_inverval_value(
                optimal_y_label_distance,
                (0.0, margined_area.height),
                (0.0, self.total_y_distance),
            );
            let mut yvs = self.min_y_value.get_values_in_between(
                &self.max_y_value,
                min_y_label_distance_mapped,
                optimal_y_label_distance_mapped,
            );
            yvs.insert(0, self.min_y_value.clone());
            yvs.push(self.max_y_value.clone());
            let yvs = yvs;
            for yv in yvs {
                let text = YD::display_value(&yv);
                let distance = self.min_y_value.distance_to(&yv);
                let y = crate::math::map_inverval_value(
                    distance,
                    (0.0, self.total_y_distance),
                    (0.0, margined_area.height),
                );
                self.draw_y_label(
                    frame,
                    padded_area,
                    margined_area.y + margined_area.height - y,
                    &text,
                );
            }

            //Draw x labels
            let min_x_label_distance = self.settings.min_x_label_distance.get(margined_area.size());
            let min_x_label_distance_mapped = crate::math::map_inverval_value(
                min_x_label_distance,
                (0.0, margined_area.width),
                (0.0, self.total_x_distance),
            );
            let optimal_x_label_distance =
                margined_area.width / (margined_area.width / min_x_label_distance).floor();
            let optimal_x_label_distance_mapped = crate::math::map_inverval_value(
                optimal_x_label_distance,
                (0.0, margined_area.width),
                (0.0, self.total_x_distance),
            );
            let mut xvs = self.min_x_value.get_values_in_between(&self.max_x_value, min_x_label_distance_mapped, optimal_x_label_distance_mapped);
            xvs.insert(0, self.min_x_value.clone());
            xvs.push(self.max_x_value.clone());
            let xvs = xvs;
            for xv in xvs {
                let text = XD::display_value(&xv);
                let distance = self.min_x_value.distance_to(&xv);
                let x = crate::math::map_inverval_value(
                    distance,
                    (0.0, self.total_x_distance),
                    (0.0, margined_area.width),
                );
                self.draw_x_label(
                    frame,
                    padded_area,
                    margined_area.x + x,
                    &text
                );
            }

            let points = self.points(margined_area.size());

            //Unreadable shit which finds the selected edge
            let selected_point_opt: Option<(&data::PlotSettings, &(Point, XD, YD))> = margined_cursor_position_opt
                .map(|margined_cursor_position| {
                    points
                        .iter()
                        .filter_map(|(settings, vec)| {
                            let iter = vec.iter();
                            let mapped = iter.map(|tuple| {
                                (tuple, margined_cursor_position.distance(tuple.0))
                            });
                            let filtered = mapped
                                .filter(|(_tuple, distance)| *distance <= 14.0);
                            filtered.min_by(|(_tuple1, f1), (_tuple2, f2)| f1.total_cmp(f2))
                                .map(|(tuple, distance)| (settings, tuple, distance))
                        })
                        .min_by(|(_settings1, _tuple1, distance1), (_settings2, _tuple2, distance2)| distance1.total_cmp(distance2))
                        .map(|(settings, tuple, _distance)| (settings, tuple))
                })
                .flatten();
            //Unreadable shit which finds the selected vertice
            let selected_plot_opt: Option<&data::PlotSettings> = selected_point_opt
                .map(|(settings, _)| settings)
                .or_else(|| margined_cursor_position_opt
                    .map(|margined_cursor_position| {
                        points
                            .iter()
                            .filter_map(|(settings, vec)| {
                                let windows = vec.windows(2);
                                let mapped = windows.map(|slice| {
                                    let (p1, _xd1, _yd1) = &slice[0];
                                    let (p2, _xd2, _yd2) = &slice[1];
                                    crate::math::point_to_interval_distance(margined_cursor_position, *p1, *p2)
                                });
                                let filtered = mapped
                                    .filter(|distance| *distance <= 6.0);
                                filtered.min_by(|f1, f2| f1.total_cmp(f2))
                                    .map(|distance| (settings, distance))
                            })
                            .min_by(|(_settings1, distance1), (_settings2, distance2)| distance1.total_cmp(distance2))
                            .map(|(settings, _distance)| settings)
                    }).flatten());
            
            selected_point_opt
                .iter()
                .for_each(|(_settings, (_p, xd, yd))| {
                    let mut content = String::new();
                    content.push_str(&xd.description());
                    content.push('\n');
                    content.push_str(&yd.description());
                    frame.fill_text(Text {
                        content,
                        position: Point::new(padded_area.width + pleft, ptop / 2.0),
                        color: theme.title_color,
                        size: 16.0,
                        horizontal_alignment: HorizontalAlignment::Right,
                        vertical_alignment: VerticalAlignment::Center,
                        ..Default::default()
                    });
                });

            frame.with_save(|frame| {
                frame.translate(Vector::new(margined_area.x, margined_area.y));
                for (plot_settings, vec) in points.iter() {
                    let line_color = plot_settings.theme.line_color;
                    let point_color = plot_settings.theme.point_color;
                    let line_selected = matches!(selected_plot_opt, Some(r) if std::ptr::eq(r, plot_settings));
                    let line_size = if line_selected {
                        plot_settings.line_size2
                    } else {
                        plot_settings.line_size1
                    };
                    let point_size = if line_selected { plot_settings.point_size2 } else { plot_settings.point_size1 };
                    let selected_point_size = plot_settings.point_size3;
                    //Draw lines
                    for slice in vec.windows(2) {
                        let (p1, _xd1, _yd1) = slice[0].to_owned();
                        let (p2, _xd2, _yd2) = slice[1].to_owned();
                        frame.stroke(
                            &Path::line(p1, p2),
                            Stroke {
                                color: line_color,
                                width: line_size,
                                ..Default::default()
                            },
                        );
                    }

                    //Draw points
                    for (p, _xd, _yd) in vec.iter() {
                        let selected = selected_point_opt
                            .map(|(_settings, (selected_point, _xd, _yd))| *selected_point == *p)
                            .unwrap_or(false);
                        let size = if selected { selected_point_size } else { point_size };
                        frame.fill(&Path::circle(*p, size), point_color);
                    }
                }
            });
        });
        vec![result]
    }

    fn update(
        &mut self,
        event: iced::canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::canvas::Cursor,
    ) -> (iced::canvas::event::Status, Option<data::Message>) {
        match event {
            iced::canvas::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                self.cache.clear();
                (iced::canvas::event::Status::Captured, None)
            }
            _ => (iced::canvas::event::Status::Ignored, None),
        }
    }

    fn mouse_interaction(
        &self,
        bounds: iced::Rectangle,
        cursor: iced::canvas::Cursor,
    ) -> iced::mouse::Interaction {
        let size = bounds.size();

        let full_area = Rectangle::new(Point::ORIGIN, size);
        let padded_area = self.settings.padding.transform(full_area);
        let margined_area = self.settings.margin.transform(padded_area);

        let cursor_position_opt = cursor.position_in(&bounds);
        let margined_cursor_position_opt = cursor_position_opt
            .map(|cp| Point::new(cp.x - margined_area.x, cp.y - margined_area.y));

        margined_cursor_position_opt
            .and_then(|cursor_position| {
                let points = self.points(margined_area.size());
                let hovered = points.iter().any(|(_settings, vec)| {
                    vec.windows(2).any(|slice| {
                        let (p1, _xd1, _yd1) = &slice[0];
                        let (p2, _xd2, _yd2) = &slice[1];
                        crate::math::point_to_interval_distance(cursor_position, *p1, *p2) <= 6.0
                            || cursor_position.distance(*p1) <= 14.0
                            || cursor_position.distance(*p2) <= 14.0
                    })
                });
                hovered.then_some(iced::mouse::Interaction::Pointer)
            })
            .unwrap_or(iced::mouse::Interaction::default())
    }
}
