use std::{fmt::Display, fs, path::PathBuf};

use m8_file_parser::{param_gatherer::{Describable, ParameterGatherer}, reader::Reader, Instrument, Version};

use crate::{types::M8FstoErr, ShowCommand, ShowTarget};

struct AsciiTherer<'a, 'writer> {
    write: &'a mut std::fmt::Formatter<'writer>,
    indent: usize
}

const BLOCKS : [char; 8] = [
    '▏',
    '▎',
    '▌',
    '▍',
    '█',
    '▋',
    '▊',
    '▉'
];

const NAME_LENGTH : usize = 9;

impl<'a, 'writer> ParameterGatherer for AsciiTherer<'a, 'writer> {
    fn hex(self, name: &str, val: u8) -> Self {
        let bar_val = val as usize + 1;

        let full = "▉".repeat(bar_val / 8);
        let middle = BLOCKS[bar_val % 8];
        let after = "░".repeat((256 - bar_val) / 8);

        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}: {val:02X} {full}{middle}{after}",
                sp="",
                idt=self.indent);
        self
    }

    fn bool(self, name: &str, val: bool) -> Self {
        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}: {val}",
                sp="",
                idt=self.indent);
        self
    }

    fn float(self, name: &str, val: f64) -> Self {
        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}: {val}",
                sp="",
                idt= self.indent);
        self
    }

    fn str(self, name: &str, val: &str) -> Self {
        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}: \"{val}\"",
                sp="",
                idt=self.indent);
        self
    }

    fn enumeration(self, name: &str, hex: u8, val: &str) -> Self {
        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}: {hex:X} {val}",
                sp="",
                idt=self.indent);
        self
    }

    fn nest_f<F>(self, name: &str, f: F) -> Self
        where F : FnOnce (Self) -> Self, Self : Sized {

        let _ =
            writeln!(self.write, "{sp:idt$}{name:NAME_LENGTH$}:",
                sp="",
                idt=self.indent);

        let pg = f(Self {
            write: self.write,
            indent: self.indent + 2
        });

        Self {
            write: pg.write,
            indent: self.indent
        }
    }
}

struct ElemDisplay<T> {
    instr: T,
    ver: Version
}

impl<T : Describable> Display for ElemDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.instr.describe(AsciiTherer { write: f, indent: 0 }, self.ver);
        Ok(())
    }
}

fn show_from_instrument(show: ShowCommand, w: &mut dyn std::io::Write, instr_eq: m8_file_parser::InstrumentWithEq) -> Result<(), M8FstoErr> {
    match show.show_command {
        ShowTarget::Song => Ok(()) ,
        ShowTarget::Chain { id: _ } => Ok(()),
        ShowTarget::Phrase { id: _} => Ok(()),
        ShowTarget::Instrument { id: _ } => {
            write!(w, "{}", ElemDisplay {
                instr: instr_eq.instrument,
                ver: instr_eq.version
            }).map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Table { id: _ } => {
            writeln!(w, "{}", instr_eq.table_view()).map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Eq { id: _ } => {
            match instr_eq.eq {
                None => writeln!(w, "No eq saved in the instrument").map_err(|_| M8FstoErr::PrintError),
                Some(equ) => {
                    write!(w, "{}", ElemDisplay {
                        instr: equ,
                        ver: instr_eq.version
                    }).map_err(|_| M8FstoErr::PrintError)
                }
            }
        },
    }
}
fn show_from_song(show: ShowCommand, w: &mut dyn std::io::Write, song: m8_file_parser::Song) -> Result<(), M8FstoErr> {
    match show.show_command {
        ShowTarget::Song => {
            writeln!(w, "{}", song.song).map_err(|_| M8FstoErr::PrintError)
        }
        ShowTarget::Chain { id } => {
            writeln!(w, "{}", song.chains[id]).map_err(|_| M8FstoErr::PrintError)
        }
        ShowTarget::Phrase { id } => {
            writeln!(w, "{}", song.phrase_view(id)).map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Instrument { id: None} => {
            for (ix, instr) in song.instruments.iter().enumerate() {
                match instr {
                    Instrument::None => {}
                    Instrument::FMSynth(fm) => {
                        writeln!(w, "{:02X} FMSynth:{}", ix, fm.name).map_err(|_| M8FstoErr::PrintError)?;
                    },
                    Instrument::WavSynth(ws) => {
                        writeln!(w, "{:02X} Wavsynth:{} (shape: {:?})", ix, ws.name, ws.shape).map_err(|_| M8FstoErr::PrintError)?;
                    }
                    Instrument::MacroSynth(ms) => {
                        writeln!(w, "{:02X} MacroSynth:{} (shape: {:?})", ix, ms.name, ms.shape).map_err(|_| M8FstoErr::PrintError)?;
                    }
                    Instrument::HyperSynth(hs) => {
                        writeln!(w, "{:02X} HyperSynth:{} ", ix, hs.name).map_err(|_| M8FstoErr::PrintError)?;
                    }
                    Instrument::Sampler(smp) => {
                        writeln!(w, "{:02X} Sampler:{} - {}", ix, smp.name, smp.sample_path).map_err(|_| M8FstoErr::PrintError)?;
                    }
                    Instrument::MIDIOut(midi) => {
                        writeln!(w, "{:02X} MIDIOut:{} (chn {}: bnk {} - prg {})", ix, midi.name, midi.channel, midi.bank_select, midi.program_change).map_err(|_| M8FstoErr::PrintError)?;
                    }
                    Instrument::External(ext) => {
                        writeln!(w, "{:02X} External:{} (chn {}: bnk {} - prg {})", ix, ext.name, ext.channel, ext.bank, ext.program).map_err(|_| M8FstoErr::PrintError)?;
                    }
                }
            };
            Ok(())
        },
        ShowTarget::Instrument { id: Some(id)} => {
            write!(w, "{}", ElemDisplay {
                instr: song.instruments[id].clone(),
                ver: song.version
            }).map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Table { id: None } => {
            writeln!(w, "Please select table number").map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Table { id: Some(id) } => {
            writeln!(w, "{}", song.table_view(id)).map_err(|_| M8FstoErr::PrintError)
        },
        ShowTarget::Eq { id: None } => {
            writeln!(w, "Please select eq number").map_err(|_| M8FstoErr::PrintError)
        }
        ShowTarget::Eq { id: Some(id) } => {
            write!(w, "{}", ElemDisplay {
                instr: song.eqs[id].clone(),
                ver: song.version
            }).map_err(|_| M8FstoErr::PrintError)
        },
    }
}

pub fn show_element(show: ShowCommand, w: &mut dyn std::io::Write) -> Result<(), M8FstoErr> {
    let song_path = PathBuf::from(show.file.clone());
    let file_blob = fs::read(song_path.clone())
        .map_err(|e|
            M8FstoErr::CannotReadFile { path: song_path.clone(), reason: format!("{:?}", e) })?;

    let mut reader = Reader::new(file_blob);

    match m8_file_parser::Song::read_from_reader(&mut reader) {
        Ok(song) => show_from_song(show, w, song),
        Err(e) => {
            reader.set_pos(0);
            match m8_file_parser::Instrument::read_from_reader(&mut reader) {
                Ok(instr_eq) => show_from_instrument(show, w, instr_eq),
                Err(_) => {
                    Err(M8FstoErr::UnparseableM8File {
                        path: song_path,
                        reason: format!("{:?}", e)
                    })
                }
            }
        }
    }
}
