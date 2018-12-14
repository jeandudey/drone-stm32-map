//! Drone STM32 SVD bindings generator.

#![feature(range_contains)]
#![feature(transpose_result)]
#![warn(missing_docs)]
#![allow(clippy::precedence)]

extern crate drone_mirror_failure as failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate xml;

mod device;

use device::Device;
use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::{env, process};
use xml::name::OwnedName;
use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriterEvent;
use xml::{EventReader, EventWriter};

const REG_EXCLUDE: &[&str] = &[
  "FPU",
  "FPU_CPACR",
  "ITM",
  "MPU",
  "NVIC",
  "SCB",
  "STK",
  "TPIU",
];

/// Returns a device string based on features.
#[macro_export]
macro_rules! svd_feature {
  () => {
    if cfg!(feature = "stm32f100") {
      "stm32f100"
    } else if cfg!(feature = "stm32f101") {
      "stm32f101"
    } else if cfg!(feature = "stm32f102") {
      "stm32f102"
    } else if cfg!(feature = "stm32f103") {
      "stm32f103"
    } else if cfg!(feature = "stm32f107") {
      "stm32f107"
    } else if cfg!(feature = "stm32l4x1") {
      "stm32l4x1"
    } else if cfg!(feature = "stm32l4x2") {
      "stm32l4x2"
    } else if cfg!(feature = "stm32l4x3") {
      "stm32l4x3"
    } else if cfg!(feature = "stm32l4x5") {
      "stm32l4x5"
    } else if cfg!(feature = "stm32l4x6") {
      "stm32l4x6"
    } else if cfg!(feature = "stm32l4r5") {
      "stm32l4r5"
    } else if cfg!(feature = "stm32l4r7") {
      "stm32l4r7"
    } else if cfg!(feature = "stm32l4r9") {
      "stm32l4r9"
    } else if cfg!(feature = "stm32l4s5") {
      "stm32l4s5"
    } else if cfg!(feature = "stm32l4s7") {
      "stm32l4s7"
    } else if cfg!(feature = "stm32l4s9") {
      "stm32l4s9"
    } else {
      ""
    }
  };
}

/// Generates code for register mappings.
pub fn generate_regs(feature: &str, pool_number: usize, pool_size: usize) {
  let run = || {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let device = svd_deserialize(feature, &out_dir)?;
    let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
    device.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)?;
    Ok::<(), Error>(())
  };
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest(feature: &str) {
  let run = || {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let device = svd_deserialize(feature, &out_dir)?;
    let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
    device.generate_rest(&mut reg_tokens, &mut interrupts, REG_EXCLUDE)?;
    Ok::<(), Error>(())
  };
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

fn svd_deserialize(feature: &str, out_dir: &Path) -> Result<Device, Error> {
  let mut svd = File::create(out_dir.join("svd.xml"))?;
  make_svd(feature, &mut svd)?;
  let mut svd = File::open(out_dir.join("svd.xml"))?;
  let mut xml = String::new();
  svd.read_to_string(&mut xml)?;
  serde_xml_rs::deserialize(xml.as_bytes()).map_err(Into::into)
}

fn make_svd(feature: &str, svd: &mut File) -> Result<(), Error> {
  if feature == "stm32f100" {
    patch("STM32F100.svd", svd, patch_stm32f1())?;
  } else if feature == "stm32f101" {
    patch("STM32F101.svd", svd, patch_stm32f1())?;
  } else if feature == "stm32f102" {
    patch("STM32F102.svd", svd, patch_stm32f1())?;
  } else if feature == "stm32f103" {
    patch("STM32F103.svd", svd, patch_stm32f1())?;
  } else if feature == "stm32f107" {
    patch("STM32F107.svd", svd, patch_stm32f1())?;
  } else if feature == "stm32l4x1" {
    patch("STM32L4x1.svd", svd, patch_stm32l4())?;
  } else if feature == "stm32l4x2" {
    patch("STM32L4x2.svd", svd, patch_stm32l4())?;
  } else if feature == "stm32l4x3" {
    patch("STM32L4x3.svd", svd, patch_stm32l4())?;
  } else if feature == "stm32l4x5" {
    patch("STM32L4x5.svd", svd, patch_stm32l4())?;
  } else if feature == "stm32l4x6" {
    patch("STM32L4x6.svd", svd, patch_stm32l4())?;
  } else if feature == "stm32l4r5" {
    patch("STM32L4R5.svd", svd, patch_stm32l4plus())?;
  } else if feature == "stm32l4r7" {
    patch("STM32L4R7.svd", svd, patch_stm32l4plus())?;
  } else if feature == "stm32l4r9" {
    patch("STM32L4R9.svd", svd, patch_stm32l4plus())?;
  } else if feature == "stm32l4s5" {
    patch("STM32L4S5.svd", svd, patch_stm32l4plus())?;
  } else if feature == "stm32l4s7" {
    patch("STM32L4S7.svd", svd, patch_stm32l4plus())?;
  } else if feature == "stm32l4s9" {
    patch("STM32L4S9.svd", svd, patch_stm32l4plus())?;
  } else {
    patch("blank.svd", svd, |o, e, _| match e {
      _ => patch_pass(o, e),
    })?;
  }
  Ok(())
}

fn patch_stm32f1() -> impl FnMut(
  &mut EventWriter<&mut File>,
  &ReaderEvent,
  &[OwnedName],
) -> Result<(), Error> {
  |o, e, _| match e {
    _ => patch_pass(o, e),
  }
}

fn patch_stm32l4() -> impl FnMut(
  &mut EventWriter<&mut File>,
  &ReaderEvent,
  &[OwnedName],
) -> Result<(), Error> {
  let mut register_name = String::new();
  move |o, e, path| match e {
    ReaderEvent::Characters(s)
      if check_path(
        path,
        &[
          "device",
          "peripherals",
          "peripheral",
          "registers",
          "register",
          "name",
        ],
      ) =>
    {
      register_name = s.clone();
      patch_pass(o, e)
    }
    ReaderEvent::Characters(s)
      if s == "SP3EN"
        && check_path(
          path,
          &[
            "device",
            "peripherals",
            "peripheral",
            "registers",
            "register",
            "fields",
            "field",
            "name",
          ],
        )
        && register_name == "APB1ENR1" =>
    {
      o.write(WriterEvent::Characters("SPI3EN"))?;
      Ok(())
    }
    _ => patch_pass(o, e),
  }
}

fn patch_stm32l4plus() -> impl FnMut(
  &mut EventWriter<&mut File>,
  &ReaderEvent,
  &[OwnedName],
) -> Result<(), Error> {
  let mut register_name = String::new();
  move |o, e, path| match e {
    ReaderEvent::StartElement { name, .. }
      if name.local_name == "peripherals" && check_path(path, &["device"]) =>
    {
      patch_pass(o, e)?;
      patch_add(o, "patch/add_dmamux.xml")
    }
    ReaderEvent::Characters(s)
      if check_path(
        path,
        &[
          "device",
          "peripherals",
          "peripheral",
          "registers",
          "register",
          "name",
        ],
      ) =>
    {
      register_name = s.clone();
      patch_pass(o, e)
    }
    ReaderEvent::Characters(s)
      if s == "SP3EN"
        && check_path(
          path,
          &[
            "device",
            "peripherals",
            "peripheral",
            "registers",
            "register",
            "fields",
            "field",
            "name",
          ],
        )
        && register_name == "APB1ENR1" =>
    {
      o.write(WriterEvent::Characters("SPI3EN"))?;
      Ok(())
    }
    _ => patch_pass(o, e),
  }
}

fn patch<
  F: FnMut(
    &mut EventWriter<&mut File>,
    &ReaderEvent,
    &[OwnedName],
  ) -> Result<(), Error>,
>(
  input: &str,
  output: &mut File,
  mut f: F,
) -> Result<(), Error> {
  let input = format!("{}/../svd_files/{}", env!("CARGO_MANIFEST_DIR"), input);
  let input = EventReader::new(BufReader::new(File::open(input)?));
  let mut output = EventWriter::new(output);
  let mut path = Vec::new();
  for event in input {
    let event = event?;
    f(&mut output, &event, &path)?;
    match &event {
      ReaderEvent::StartElement { name, .. } => {
        path.push(name.clone());
      }
      ReaderEvent::EndElement { name, .. } => {
        let tail = path.pop();
        assert_eq!(tail.as_ref(), Some(name));
      }
      _ => {}
    }
  }
  Ok(())
}

fn patch_pass(
  output: &mut EventWriter<&mut File>,
  event: &ReaderEvent,
) -> Result<(), Error> {
  event
    .as_writer_event()
    .map(|x| output.write(x))
    .transpose()?;
  Ok(())
}

fn patch_add(
  output: &mut EventWriter<&mut File>,
  patch: &str,
) -> Result<(), Error> {
  let patch = format!("{}/../svd_files/{}", env!("CARGO_MANIFEST_DIR"), patch);
  for e in EventReader::new(BufReader::new(File::open(patch)?)) {
    match e? {
      ReaderEvent::StartDocument { .. } | ReaderEvent::EndDocument => {}
      e => patch_pass(output, &e)?,
    }
  }
  Ok(())
}

fn check_path(a: &[OwnedName], b: &[&str]) -> bool {
  a.len() == b.len()
    && a
      .iter()
      .zip(b.iter())
      .try_for_each(|(a, &b)| if a.local_name == b { Some(()) } else { None })
      .is_some()
}
