use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;
use opencv::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;
const PRG: &str = "starrynight";
//const EMPTY: &str = "tests/inputs/empty.txt";
//const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
//#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    //let expected = format!("{}: .* [(]os error 2[)]", bad);
    let expected = format!("File: '{}'. not found\n", bad);
    println!("EXPECTED:  {}", expected);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::contains("not found".to_string()));
        //.stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn test_cvmat_eye_at() -> TestResult{
    let eye = Mat::eye(3,3,f64::typ())? ;
    let eye = eye.to_mat()?;
    assert_eq!(1.0, *eye.at::<f64>(0)?);
    assert_eq!(0.0, *eye.at::<f64>(1)?);
    assert_eq!(0.0, *eye.at::<f64>(2)?);

    assert_eq!(0.0, *eye.at::<f64>(3)?);
    assert_eq!(1.0, *eye.at::<f64>(4)?);    
    assert_eq!(0.0, *eye.at::<f64>(5)?);

    assert_eq!(0.0, *eye.at::<f64>(6)?);
    assert_eq!(0.0, *eye.at::<f64>(7)?);
    assert_eq!(1.0, *eye.at::<f64>(8)?);
    // that was awkward ^

    assert_eq!(1.0, *eye.at_2d::<f64>(0,0)?);
    assert_eq!(1.0, *eye.at_2d::<f64>(1,1)?);
    assert_eq!(1.0, *eye.at_2d::<f64>(2,2)?);
    // slightly more intuitive ? ^

    let _zeros = Mat::zeros(4, 4, f64::typ())?.to_mat();

    Ok(())  

}