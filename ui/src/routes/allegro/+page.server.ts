import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { writeFile } from "fs/promises";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export const actions: Actions = {
  addrelease: async ({ request, cookies }) => {
    // get form data
    const data = await request.formData();
    const name = data.get("name");
    const artist = data.get("artist");
    const description = data.get("description");
    const image = data.get("file") as File;
    const token = cookies.get("token");

    // ensure name is not empty
    if (name === "" || name === null) {
      return fail(400, { error: "Name cannot be empty" });
    }

    // ensure artist is not empty
    const artist_id = Number(artist);
    if (isNaN(artist_id) || artist_id <= 0) {
      return fail(400, { error: "Artist cannot be empty" });
    }

    try {
      const has_image = image.size > 0;
      const response = await api.post("/music/add/release", {
        name,
        artist_id,
        description,
        has_image,
        token,
      });

      console.log(response.data);

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
    const token = cookies.get("token");

    // ensure name is not empty
    if (name === "" || name === null) {
      console.log("error reached");
      return fail(400, { error: "Name cannot be empty" });
    }

    try {
      const has_image = image.size > 0;
      const response = await api.post("/music/add/artist", {
        name,
        description,
        has_image,
        token,
      });

      console.log(response.data);

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
