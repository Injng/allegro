export type Release = {
  name: string;
  artist: string;
  cover: string;
  type: "album" | "single";
};

export type Artist = {
  id: number;
  name: string;
  description: string | null;
  image_path: string | null;
};
