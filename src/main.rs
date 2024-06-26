use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read};
use csv::Writer;
use encoding_rs::EUC_KR;

fn run() -> Result<(), Box<dyn Error>> {
    // 입력 파일과 출력 파일 경로를 설정합니다.
    let input_file = "src/mart_djy_03.txt";
    let output_file = "src/output3.csv";

    // 용도별 해더
    let header_energy =
        [
        "date", "deaji", "doro", "sigungu_cd", "bjdong_cd", "san", "bun",
        "ji", "new_add", "new_add_1", "new_add_2", "new_add_3", "new_add_4", "energy_use",
        ];

    let header_building_info=
        [
            "deajaing_pk", "deaji", "doro", "building_name", "sigungu_cd", "bjdong_cd", "san",
            "bun", "ji", "land_area", "building_area", "building_coverage_ratio", "total_floor_area",
            "total_floor_area_by_floor_area_ratio", "floor_area_ratio", "main_structure_information",
            "main_structure_name", "main_purpose_information", "main_purpose_name", "household",
            "family_count", "height", "floor_ground_count", "floor_underground_count",
            "permission_use_date", "unit", "energy_efficiency_grade", "energy_saving_rate",
            "epi_score", "eco_building_grade", "eco_building_score", "intelligent_building_grade",
            "intelligent_building_score",
        ];

    // 필터링할 인덱스 정의
    let filter_index = vec![
        1, 2, 3, 4, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 33, 36, 37, 38, 39,
        45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 61, 62, 63, 64, 65,
        74, 75, 76
    ];

    //let filter_index = vec![];

    // 출력 파일이 존재하지 않으면 생성합니다.
    let mut output_file_handle = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file)?;

    // 입력 파일을 엽니다.
    let mut file = File::open(input_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // EUC-KR에서 UTF-8로 변환합니다.
    let (decoded, _, _) = EUC_KR.decode(&buffer);
    let decoded_str = decoded.into_owned();

    // BufReader를 사용하여 줄 단위로 읽습니다.
    let reader = BufReader::new(decoded_str.as_bytes());

    // 출력 CSV 파일을 엽니다.
    let mut wtr = Writer::from_path(output_file)?;

    // CSV 파일에 헤더를 작성합니다.
    wtr.write_record(header_building_info)?;

    // 입력 파일의 각 줄을 읽고 처리합니다.
    let mut row_count = 0;
    for line in reader.lines() {
        let line = line?;
        // 마지막 '|' 문자를 제거합니다.
        let line = line.trim_end_matches('|');

        // '|' 문자를 기준으로 문자열을 분리합니다.
        let fields: Vec<&str> = line.split('|').collect();

        let filtered_fields: Vec<&str> = if filter_index.is_empty() {
            fields.clone()
        } else {
            fields.iter()
                .enumerate()
                .filter(|&(index, _)| !filter_index.contains(&index))
                .map(|(_, &field)| field)
                .collect()
        };

        // CSV 파일에 행을 작성합니다.
        wtr.write_record(&filtered_fields)?;
        row_count += 1;
    }

    // CSV 파일을 플러시하여 완료합니다.
    wtr.flush()?;
    println!("TXT 파일이 CSV 파일로 변환되었습니다: {}", output_file);
    println!("총 {}개의 행이 처리되었습니다.", row_count);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("오류 발생: {}", err);
    }
}