use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_pubx<'a, 'b>(
    sentence: (
        Option<GpsTime>,
        Option<GpsPosition>,
        Option<f32>,
        &'b [u8],
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<u8>,
        bool,
    ),
) -> Result<PubxLocData, NmeaSentenceError<'a>> {
    Ok(PubxLocData {
        time: sentence.0,
        position: sentence.1,
        altitude: sentence.2,
        status: NavStat::try_from(sentence.3)?,
        h_acc: sentence.4,
        v_acc: sentence.5,
        sog: sentence.6,
        cog: sentence.7,
        v_vel: sentence.8,
        diff_age: sentence.9,
        hdop: sentence.10,
        vdop: sentence.11,
        tdop: sentence.12,
        num_satellites: sentence.13,
        dead_reckoning: sentence.14,
    })
}

named!(pub (crate) parse_pubx<PubxLocData>,
    map_res!(
        do_parse!(
            tag!(b"00") >>
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            position: opt!(complete!(parse_gps_position)) >>
            char!(',') >>
            altitude: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            status: take!(2) >>
            char!(',') >>
            h_acc: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            v_acc: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            sog: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            cog: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            v_vel: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            diff_age: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            hdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            vdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            tdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            num_satellites: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            take_until!(",") >> // Skip reserved
            dead_reckoning: alt!(map!(char!('0'), |_| false) | map!(char!('1'), |_| true)) >>
            char!('*') >>
            (time, position, altitude, status, h_acc, v_acc, sog, cog, v_vel, diff_age, hdop, vdop, tdop, num_satellites, dead_reckoning)
        ),
        build_pubx
    )
);
