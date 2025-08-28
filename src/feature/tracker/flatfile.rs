//! A filesystem tracker

// flat file tracker
// 2 files
// - lock  "lock file" tracker is running
// datbase file: JSON doc

use std::io::{Read, Write};
use std::path::Path;
use std::{fs::OpenOptions, path::PathBuf};

use error_stack::ResultExt;
use error_stack::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::tracker::{EndTime, StartTime};
use crate::feature::tracker::TimeRecord;


#[derive(Debug, thiserror::Error)]
#[error("FlatFileTracker error")]
pub struct FlatFileTrackerError;

pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LockfileData {
    start_time: StartTime
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlatFileDatabase {
    records: Vec<TimeRecord>
}

impl FlatFileDatabase {
    pub fn push(&mut self, value: TimeRecord) {
        self.records.push(value)
    }
}

impl FlatFileTracker {
    fn new<D,L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>
        {
            let db = db.into();
            let lockfile = lockfile.into();
            Self {db, lockfile}
        }
        
        fn start(&self) -> Result<(), FlatFileTrackerError> {
            let lockfile_data = {
                let start_time = StartTime::now();
                let data = LockfileData {
                    start_time
                };
                serde_json::to_string(&data).change_context(FlatFileTrackerError)
                .attach_printable("failed to attach lockfile data")?
            };
            


            //save current start time into lockfile
            OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile when starting tracker")?
            .write_all(lockfile_data.as_bytes())
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile when starting tracker")?;
        
            Ok(())
        }


        

        fn is_running(&self) -> bool {
            self.lockfile.exists()
        }
        
        fn stop(&self) -> Result<(), FlatFileTrackerError> {
            //read time from the lockfile
            let start = read_lockfile(self.lockfile)?.start_time;
            //get end 
            
            // 3. create record to the database file
            let end = EndTime::now();

            let record = TimeRecord { start, end };

            let mut db = load_database(&self.db)?;
            db.push(record);
            save_database(&self.db, &db)?;



            std::fs::remove_file(&self.lockfile)
                .change_context(FlatFileTrackerError)
                .attach_printable("unable to remove lockfile when stopping tracker")
        }
        fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, FlatFileTrackerError>{
            // read the db file and return an iterator over the records
            // load records and rturn an iterator
            Ok(vec![].into_iter())
        }
}

fn save_database<P>(path: P, db: &FlatFileDatabase) -> Result<(), FlatFileTrackerError> 
where 
    P: AsRef<Path> {
        let db = serde_json::to_string(&db)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to serialize database")?;

    OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(false)
    .open(path.as_ref())
    .change_context(FlatFileTrackerError)
    .attach_printable("failed to open database")?
    .write_all(&db.as_bytes())
    .change_context(FlatFileTrackerError)
    .attach_printable("failed to write database")?;
Ok(())
    
}

fn load_database<P>(db: P) -> Result<FlatFileDatabase, FlatFileTrackerError>
where P: AsRef<Path> {

    let mut db_buf = String::default();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to open database")?
        .read_to_string(&mut db_buf)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to read database")?;

    if db_buf.is_empty( ) {
        Ok(FlatFileDatabase::default())
    }
     else {
        Ok(serde_json::from_str(&db_buf)
            .change_context(FlatFileTrackerError)
            .attach_printable("failed to deserialize database")?)
     }

    
}

fn read_lockfile<P>(lockfile: P) -> Result<LockfileData, FlatFileTrackerError>
where P: AsRef<Path>
{
    let file = OpenOptions::new()
    .read(true)
    .open(lockfile.as_ref())
    .change_context(FlatFileTrackerError)
    .attach_printable("failed to open lockfile")?;

    Ok(serde_json::from_reader(file)
.change_context(FlatFileTrackerError)
.attach_printable("failed to deserialize lockfile")?)
}


#[cfg(test)]
mod tests {
    use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};

    use super::*;

        fn tracking_paths() -> (TempDir, ChildPath, ChildPath) {
        let temp = TempDir::new().unwrap();
        let db = temp.child("db.json");
        let lockfile = temp.child("lockfile");
        (temp, db, lockfile)
    }
    fn new_flat_file_tracker(db: &ChildPath, lockfile: &ChildPath) -> FlatFileTracker {
        // Create a new FlatFileTracker with the given db and lockfile paths
        FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf())
    }

    #[test]
    fn is_running_returns_true_after_Starting_tracker() {
        let (_tempdir, db, lockfile) = tracking_paths();
        // given a default tracker
        let tracker = new_flat_file_tracker(&db, &lockfile);

        tracker.start().unwrap();

        assert!(tracker.is_running());
        // when the tracker is stared
        // wen the tracker is runnning
    }


    
    #[test]
    fn is_running_returns_false_after_Stopping_tracker() {
        // given a default tracker
        let (_tempdir, db, lockfile) = tracking_paths();
        // given a default tracker
        let tracker = new_flat_file_tracker(&db, &lockfile);

        tracker.stop();

        assert!(!tracker.is_running());
        // when the tracker is stared
        // wen the tracker is runnning
    }

    #[test]
    fn time_record_created_when_tracking_stops() {
        // given a default tracker
        let (_tempdir, db, lockfile) = tracking_paths();
        // given a default tracker
        let tracker: FlatFileTracker = new_flat_file_tracker(&db, &lockfile);

         std::thread::sleep(std::time::Duration::from_millis(20));
        tracker.stop().unwrap();

        assert!(tracker.records().unwrap().next().is_some());
        // when the tracker is stared
    }

}