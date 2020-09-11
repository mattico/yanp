use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

named!(pub (crate) parse_vtg<VtgData>,
    map_res!(
        do_parse!(
            bearing_true: opt!(terminated!(map_res!(take_until!(","), parse_num::<f32>), tag!(",T,"))) >>
            bearing_magnetic: opt!(terminated!(map_res!(take_until!(","), parse_num::<f32>), tag!(",M,"))) >>
            speed_knots: opt!(terminated!(map_res!(take_until!(","), parse_num::<f32>), tag!(",N,"))) >>
            speed_kmh: opt!(terminated!(map_res!(take_until!(","), parse_num::<f32>), tag!(",K*"))) >>
            (bearing_true, bearing_magnetic, speed_knots, speed_kmh)
        ),
        | sentence: (Option<f32>, Option<f32>, Option<f32>, Option<f32>)| -> Result<VtgData, NmeaSentenceError> {
            Ok(VtgData{
                bearing_true: sentence.0,
                bearing_magnetic: sentence.1,
                speed_knots: sentence.2,
                speed_kmh: sentence.3,
            })
        }
    )
);
