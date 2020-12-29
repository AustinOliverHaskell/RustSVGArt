use crate::parser::parser_defs;

pub fn parse_svg_path(element: &quick_xml::events::BytesStart) -> Result<parser_defs::SVGShape, String> {
    let attribute_list = element.attributes();

    let mut color: parser_defs::SVGColor = parser_defs::SVGColor {r: 0, g: 0, b: 0, a: 0};
    let mut position: Option<parser_defs::SVGPoint> = None;
    let mut points: Vec<parser_defs::SVGPoint> = Vec::new(); 
    for attribute in attribute_list {
        let att = attribute.unwrap();
        match att.key {
            b"d" => {
                let data = &String::from_utf8(att.value.to_vec()).unwrap();

                let possible_position = parse_position(data);
                match possible_position {
                    Err(e) => return Err(e),
                    Ok(val) => position = Some(val)
                }

                let possible_points = parse_points(data, position.unwrap());
                match possible_points {
                    Err(e) => return Err(e),
                    Ok(val) => points = val 
                }
            },
            b"stroke" => color = parser_defs::map_color(&String::from_utf8(att.value.to_vec()).unwrap()),
            _ => print!("")
        }
    }

    Ok(parser_defs::SVGShape {
        points: points,
        position: position.unwrap(),
        shape_type: String::from("polyline"),
        color: color
    })
}

// Note: It is very inefficiant to search through this twice, this could be parsed at the same
//  time as the point data to avoid the extra loop. - Austin Haskell
fn parse_position(data: &str) -> Result<parser_defs::SVGPoint, String> {
    let items: Vec<&str> = data.split(' ').collect();

    let mut point: Option<parser_defs::SVGPoint> = None;
    for item in items {
        if item.chars().nth(0).unwrap() == 'M' {
            let positions: Vec<&str> = (&item[1..]).split(',').collect();
            if positions.len() < 2 {
                return Err(String::from("Error: Malformed positional data for path. "));
            }

            point = Some(
                parser_defs::SVGPoint {
                    x: positions[0].parse().unwrap(),
                    y: positions[1].parse().unwrap()
                }
            );
        }
    }
    if point.is_none() {
        return Err(String::from("Error: No position data found (M), svg file is considered malformed. "));
    }

    Ok(point.unwrap())
}

fn parse_points(data: &str, position: parser_defs::SVGPoint) -> Result<Vec<parser_defs::SVGPoint>, String> {
    let items: Vec<&str> = data.split(' ').collect();

    let mut point_list: Vec<parser_defs::SVGPoint> = Vec::new();
    for item in items {
        if item.chars().nth(0).unwrap() == 'l' {
            let coordanates: Vec<&str> = (&item[1..]).split(',').collect();

            let x: f32 = coordanates[0].parse().unwrap();
            let y: f32 = coordanates[1].parse().unwrap();
            point_list.push(parser_defs::SVGPoint {
                x: position.x + x,
                y: position.y + y
            });
        }
    }
    if point_list.is_empty() {
        return Err(String::from("Error: No point data found (l), svg file is considered malformed. "));
    }

    Ok(point_list)
}