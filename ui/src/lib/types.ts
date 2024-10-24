export type Release = {
  id: number;
  name: string;
  performer: Artist;
  description: string | null;
  image_path: string | null;
};

export type Piece = {
  id: number;
  name: string;
  movements: number | null;
  composer: Artist;
  songwriter: Artist | null;
  description: string | null;
};

export type Artist = {
  id: number;
  name: string;
  description: string | null;
  artist_type: "performer" | "composer" | "songwriter";
  image_path: string | null;
};
