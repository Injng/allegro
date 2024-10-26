import type {
  DbRecording,
  DbRelease,
  DbPiece,
  Recording,
  Release,
  Piece,
  Artist,
} from "$lib/types";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export async function buildArtist(
  id: number,
  type: "performer" | "composer" | "songwriter",
): Promise<Artist> {
  const response = await api.post(`/music/get/${type}`, { id });
  const data = response.data.message;

  return {
    id: data.id,
    name: data.name,
    description: data.description,
    artistType: type,
    imagePath: data.image_path,
  };
}

export async function buildPiece(dbPiece: DbPiece): Promise<Piece> {
  // Get composers
  const composers = await Promise.all(
    dbPiece.composer_ids.map((id) => buildArtist(id, "composer")),
  );

  // Get songwriters if they exist
  const songwriters = dbPiece.songwriter_ids
    ? await Promise.all(
        dbPiece.songwriter_ids.map((id) => buildArtist(id, "songwriter")),
      )
    : null;

  return {
    id: dbPiece.id,
    name: dbPiece.name,
    movements: dbPiece.movements,
    composers,
    songwriters,
    description: dbPiece.description,
  };
}

export async function buildRelease(dbRelease: DbRelease): Promise<Release> {
  // Get performers
  const performers = await Promise.all(
    dbRelease.performer_ids.map((id) => buildArtist(id, "performer")),
  );

  return {
    id: dbRelease.id,
    name: dbRelease.name,
    performers,
    description: dbRelease.description,
    imagePath: dbRelease.image_path,
  };
}

export async function buildRecording(
  dbRecording: DbRecording,
): Promise<Recording> {
  // Get the piece
  const pieceResponse = await api.post("/music/get/piece", {
    id: dbRecording.piece_id,
  });
  const piece = await buildPiece(pieceResponse.data.message);

  // Get the release
  const releaseResponse = await api.post("/music/get/release", {
    id: dbRecording.release_id,
  });
  const release = await buildRelease(releaseResponse.data.message);

  // Get performers
  const performers = await Promise.all(
    dbRecording.performer_ids.map((id) => buildArtist(id, "performer")),
  );

  return {
    id: dbRecording.id,
    pieceName: dbRecording.piece_name,
    piece,
    release,
    performers,
    trackNumber: dbRecording.track_number,
    filePath: dbRecording.file_path,
  };
}
