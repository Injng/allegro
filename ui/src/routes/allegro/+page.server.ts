import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { writeFile } from "fs/promises";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export const actions: Actions = {
  addartist: async ({ request, cookies }) => {
    const data = await request.formData();
    const name = data.get("name");
    const description = data.get("description");
    const image = data.get("file") as File;
    const token = cookies.get("token");

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
      return redirect(303, "/");
    }
  },
};
