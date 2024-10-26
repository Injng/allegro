export type DbRecording = {
  id: number;
  piece_name: string;
  piece_id: number;
  file_path: string;
  release_id: number;
  track_number: number;
  performer_ids: number[];
};

export type Recording = {
  id: number;
  pieceName: string;
  piece: Piece;
  release: Release;
  performers: Artist[];
  trackNumber: number;
  filePath: string;
};

export type DbRelease = {
  id: number;
  name: string;
  description: string | null;
  image_path: string | null;
  recording_ids: number[] | null;
  performer_ids: number[];
};

export type Release = {
  id: number;
  name: string;
  performers: Artist[];
  description: string | null;
  imagePath: string | null;
};

export type DbPiece = {
  id: number;
  name: string;
  movements: number | null;
  description: string | null;
  composer_ids: number[];
  songwriter_ids: number[] | null;
};

export type Piece = {
  id: number;
  name: string;
  movements: number | null;
  composers: Artist[];
  songwriters: Artist[] | null;
  description: string | null;
};

export type Artist = {
  id: number;
  name: string;
  description: string | null;
  artistType: "performer" | "composer" | "songwriter";
  imagePath: string | null;
};

export interface TokenData {
  token: string;
}
