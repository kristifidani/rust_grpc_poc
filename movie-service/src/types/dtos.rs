pub const DB_NAME: &str = "movies";

#[derive(Debug)]
pub struct MovieEntity {
    // Unique uuid index
    pub index: String,
    pub title: String,
    pub year: i32,
    pub genre: String,
}

impl From<MovieEntity> for crate::types::grpc::movie::MovieItem {
    fn from(entity: MovieEntity) -> Self {
        crate::types::grpc::movie::MovieItem {
            index: entity.index,
            title: entity.title,
            year: entity.year,
            genre: entity.genre,
        }
    }
}

impl From<crate::types::grpc::movie::MovieItem> for MovieEntity {
    fn from(item: crate::types::grpc::movie::MovieItem) -> Self {
        MovieEntity {
            index: item.index,
            title: item.title,
            year: item.year,
            genre: item.genre,
        }
    }
}
