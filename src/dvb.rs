use std::fs::File;
use std::io::Read;
use std::path::{Path};
use url::{Url};
use std::collections::HashMap;
use rustc_serialize::json::Json;
use hyper::{Client, Ok};
use hyper::status::StatusCode;
use datetime::local::*;

#[allow(unused)]
fn request(url: Url) -> Result<String, StatusCode>
{
    let mut client = Client::new();
    let mut res = client.get(url).send().unwrap();
    match res.status
    {
        StatusCode::Ok => {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            return Result::Ok(body)
        },
        _ => (Err(res.status)),
    }
    // Read the Response.
}

fn url(url:String) -> Url
{
    Url::parse( &(url) ).unwrap()
}

#[allow(unused)]
fn get_content_offline() -> String
{
    let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    //let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    let mut string = String::new();
    file.read_to_string(&mut string).ok().expect("read error");
    string
}

pub fn get_monitor(station: &str, city:&str) -> Result<Json, StatusCode>
{
    let base_url = "http://widgets.vvo-online.de/abfahrtsmonitor/Abfahrten.do";
    let url = url(format!("{base}?ort={city}&vz=0&hst={station}",
                          base=base_url,
                          city=city,
                          station=station));
    request(url).map(|str| Json::from_str(&str).unwrap())
}

pub fn find_station(search_term:&str) -> Result<Json, StatusCode>
{
    //let base_url = "http://efa.vvo-online.de:8080/dvb/XML_STOPFINDER_REQUEST";
    let base_url = "http://efa.faplino.de/dvb/XML_STOPFINDER_REQUEST";
    let url = url(format!("{base}?locationServerActive={lsa}\
                &outputFormat=JSON\
                &type_sf=any\
                &name_sf={search}\
                &coordOutputFormat=WGS84\
                &coordOutputFormatTail=0",
                base=base_url, lsa=1, search=search_term));
    request(url).map(|str| Json::from_str(&str).unwrap())
}

pub enum DepArr{ Dep, Arr }
pub struct RouteRequest
{
    pub origin:&'static str,
    pub destination:&'static str,
    pub city_origin:&'static str,
    pub city_destination:&'static str,
    pub time:LocalDateTime,
    pub deparr:DepArr
}

impl Default for RouteRequest
{ fn default() -> RouteRequest{ RouteRequest{city_origin:"Dresden", city_destination:"Dresden", origin:"", destination:"", time:LocalDateTime::now(), deparr:DepArr::Dep} }}

impl RouteRequest
{
    pub fn from_to(origin:&'static str, destination:&'static str) -> RouteRequest{
        RouteRequest{
            origin:origin,
            destination:destination,
            .. RouteRequest::default()
        }
    }
}

impl ToString for DepArr
{
    fn to_string(&self) -> String
    {
        match self {
            &DepArr::Dep => "dep".to_owned(),
            &DepArr::Arr => "arr".to_owned()
        }
    }
}

//pub fn get_route(origin:&str, destination:&str, city_origin:&str, city_destination:&str, time:i32, deparr:&str)
pub fn get_route(route:RouteRequest)
    //-> Result<Json, StatusCode>
{
    //let base_url = "http://efa.vvo-online.de:8080/dvb/XML_TRIP_REQUEST2";
    let base_url = "http://efa.faplino.de/dvb/XML_TRIP_REQUEST2";

    let deparr = "dep"; //"arr", "dep"
    
    let url =
    // {{{
        url(format!("{base}?\
                sessionID=0\
                &requestID=0\
                &language=de\
                &execInst=normal\
                &command=\
                &ptOptionsActive=-1\
                &itOptionsActive=\
                &itDateDay={day}\
                &itDateMonth={month}\
                &itDateYear={year}\
                &place_origin={city_origin}\
                &placeState_origin=empty\
                &type_origin=stop\
                &name_origin={origin}\
                &nameState_origin=empty\
                &place_destination={city_destination}\
                &placeState_destination=empty\
                &type_destination=stop\
                &name_destination={destination}\
                &nameState_destination=empty\
                &itdTripDateTimeDepArr={deparr}\
                &itdTimeHour={hour}\
                &idtTimeMinute={minute}\
                &outputFormat=JSON\
                &coordOutputFormat=WGS84\
                &coordOutputFormatTail=0",                  

                base             = base_url,
                deparr           = route.deparr.to_string(),
                year             = route.time.year(),
                month            = route.time.month() as i32 + 1, // january = 0
                day              = route.time.day(),
                hour             = route.time.hour(),
                minute           = route.time.minute(),
                city_origin      = route.city_origin ,
                city_destination = route.city_destination ,
                origin           = route.origin ,
                destination      = route.destination ));
//}}}
    println!("{:?}", url.query_pairs().unwrap());
    //request(url).map(|str| Json::from_str(&str).unwrap())
}

pub fn group_by_line(data: &Json) -> HashMap<String, Vec<String>>
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(list) = data.as_array() {
        for arrival in list {
            let line = arrival[0].as_string().unwrap();
            let direction = arrival[1].as_string().unwrap();
            let route = format!("{} {}", line.to_owned(),  direction);
            let minutes_string = arrival[2].as_string().unwrap();
            //let minutes = time::Duration::minutes(minutes_string.parse::<i64>().unwrap_or(-1));
            let wait = format!("{}min ", minutes_string);

            let mut list = map.entry(route).or_insert(vec![]);
            list.push(wait);
        }
    }
    return map;
}
