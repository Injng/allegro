import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { writeFile } from "fs/promises";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export const actions: Actions = {
  addpiece: async ({ request, cookies }) => {
    // get form data
    const data = await request.formData();
    const name = data.get("name");
    let movements: number | null = Number(data.get("movements"));
    const composerIds =
      data.get("composers")?.toString().split(",").map(Number) || [];
    const songwriterIds =
      data.get("songwriters")?.toString().split(",").map(Number) || null;
    const description = data.get("description");
    const token = cookies.get("token");

    // ensure name is not empty
    if (name === "" || name === null) {
      return fail(400, { error: "Name cannot be empty" });
    }

    // ensure artist is not empty
    if (composerIds.length === 0) {
      return fail(400, { error: "Composer cannot be empty" });
    }

    // make movements optional
    if (isNaN(movements) || movements <= 1) {
      movements = null;
    }

    try {
      const response = await api.post("/music/add/piece", {
        name,
        movements,
        composer_ids: composerIds,
        songwriter_ids: songwriterIds,
        description,
        token,
      });

      if (!response.data.success) {
        return fail(400, response.data.message);
      }

      return { success: true };
    } catch {
      return fail(400, { error: "Server error" });
    }
  },

  addrelease: async ({ request, cookies }) => {
    // get form data
    const data = await request.formData();
    const name = data.get("name");
    const performerIds =
      data.get("performers")?.toString().split(",").map(Number) || [];
    const description = data.get("description");
    const image = data.get("file") as File;
    const token = cookies.get("token");

    // ensure name is not empty
    if (name === "" || name === null) {
      return fail(400, { error: "Name cannot be empty" });
    }

    // ensure artist is not empty
    if (performerIds.length === 0) {
      return fail(400, { error: "Performer cannot be empty" });
    }

    try {
      const hasImage = image.size > 0;
      const response = await api.post("/music/add/release", {
        name,
        performer_ids: performerIds,
        description,
        has_image: hasImage,
        token,
      });

      if (!response.data.success) {
        return fail(400, response.data.message);
      }

      // upload image
      let imagePath = "";
      if (image.size > 0) {
        const fileName = response.data.message;
        imagePath = `static/uploads/${fileName}`;
        await writeFile(imagePath, Buffer.from(await image.arrayBuffer()));
      }

      return { success: true };
    } catch {
      return fail(400, { error: "Server error" });
    }
  },

  addartist: async ({ request, cookies }) => {
    // get form data
    const data = await request.formData();
    const name = data.get("name");
    const description = data.get("description");
    const image = data.get("file") as File;
    const artist_type = data.get("type");
    const token = cookies.get("token");

    // ensure name is not empty
    if (name === "" || name === null) {
      return fail(400, { error: "Name cannot be empty" });
    }

    // ensure type is not empty
    if (artist_type === "" || artist_type === null) {
      return fail(400, { error: "Type cannot be empty" });
    }

    try {
      const hasImage = image.size > 0;
      const response = await api.post("/music/add/artist", {
        name,
        description,
        has_image: hasImage,
        artist_type,
        token,
      });

      if (!response.data.success) {
        return fail(400, response.data.message);
      }

      // upload image
      let imagePath = "";
      if (image.size > 0) {
        const fileName = response.data.message;
        imagePath = `static/uploads/${fileName}`;
        await writeFile(imagePath, Buffer.from(await image.arrayBuffer()));
      }

      return { success: true };
    } catch {
      return fail(400, { error: "Server error" });
    }
  },
};
