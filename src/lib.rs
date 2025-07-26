use std::io;

use io::Seek;
use io::SeekFrom;

use std::path::Path;

use std::sync::Arc;

use std::fs::File;

use io::Read;
use io::Write;

use arrow::record_batch::RecordBatch;

use arrow_schema::Schema;

use arrow_array::RecordBatchWriter;

use arrow_csv::ReaderBuilder;
use arrow_csv::reader::Format;
use arrow_csv::reader::Reader;

use arrow_json::LineDelimitedWriter;

pub fn csv2batch_iter<R>(
    rdr: R,
    s: Arc<Schema>,
) -> Result<impl Iterator<Item = Result<RecordBatch, io::Error>>, io::Error>
where
    R: Read,
{
    let builder = ReaderBuilder::new(s).with_header(true);
    let built: Reader<_> = builder.build(rdr).map_err(io::Error::other)?;
    Ok(built.map(|r| r.map_err(io::Error::other)))
}

pub fn write_all<I, W>(b: I, wtr: &mut W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<RecordBatch, io::Error>>,
    W: RecordBatchWriter,
{
    for ritem in b {
        let item: RecordBatch = ritem?;
        wtr.write(&item).map_err(io::Error::other)?;
    }
    Ok(())
}

pub fn new_json_writer<W>(wtr: W) -> LineDelimitedWriter<W>
where
    W: Write,
{
    LineDelimitedWriter::new(wtr)
}

pub fn guess_schema<R>(mut rdr: R, max: usize) -> Result<Schema, io::Error>
where
    R: Read,
{
    let (schema, _) = Format::default()
        .with_header(true)
        .infer_schema(&mut rdr, Some(max))
        .map_err(io::Error::other)?;
    Ok(schema)
}

pub fn csv2json<R, W>(rdr: R, s: Arc<Schema>, wtr: W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let rows = csv2batch_iter(rdr, s)?;
    let mut jwtr = new_json_writer(wtr);
    write_all(rows, &mut jwtr)
}

pub fn csv_to_json<P, W>(filename: P, wtr: W, max: usize) -> Result<(), io::Error>
where
    W: Write,
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;

    let schema: Schema = guess_schema(&mut file, max)?;

    file.seek(SeekFrom::Start(0))?;

    csv2json(file, schema.into(), wtr)
}

pub fn csv2json2stdout<P>(filename: P, max: usize) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let mut stdout = io::stdout();
    csv_to_json(&filename, &mut stdout, max)?;
    stdout.flush()
}
