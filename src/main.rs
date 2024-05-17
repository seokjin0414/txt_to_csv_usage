use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read};
use csv::Writer;
use encoding_rs::EUC_KR;

fn run() -> Result<(), Box<dyn Error>> {
    // 입력 파일과 출력 파일 경로를 설정합니다.
    let input_file = "src/mart_key_01_2023.txt";
    let output_file = "src/output1.csv";

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
    wtr.write_record(&[
        "date", "deaji", "doro", "sigungu_cd", "bjdong_cd", "san", "bun", "ji",
        "new_add", "new_add_1", "new_add_2", "new_add_3", "new_add_4", "energy_use"
    ])?;

    // 입력 파일의 각 줄을 읽고 처리합니다.
    for line in reader.lines() {
        let line = line?;
        // 마지막 '|' 문자를 제거합니다.
        let line = line.trim_end_matches('|');
        // '|' 문자를 기준으로 문자열을 분리합니다.
        let fields: Vec<&str> = line.split('|').collect();
        // CSV 파일에 행을 작성합니다.
        wtr.write_record(&fields)?;
    }

    // CSV 파일을 플러시하여 완료합니다.
    wtr.flush()?;
    println!("TXT 파일이 CSV 파일로 변환되었습니다: {}", output_file);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("오류 발생: {}", err);
    }
}