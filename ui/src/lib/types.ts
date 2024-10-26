export type Recording = {
  id: number;
  pieceName: string;
  piece: Piece;
  release: Release;
  performers: Artist[];
  trackNumber: number;
  filePath: string;
};

export type Release = {
  id: number;
  name: string;
  performers: Artist[];
  description: string | null;
  imagePath: string | null;
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
