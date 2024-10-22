import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export const actions: Actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get("username");
    const password = data.get("password");

    try {
      const response = await api.post("/auth/login", { username, password });
      const token = response.data.token;

      // Set the token as a cookie
      cookies.set("token", token, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        secure: process.env.NODE_ENV === "production",
        maxAge: 60 * 60 * 24 * 7, // 1 week
      });
      console.log("reached");

      return redirect(303, "/allegro");
    } catch {
      return redirect(303, "/");
      // return fail(400, { error: "Invalid credentials" });
    }
  },

  createAdmin: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get("username");
    const password = data.get("password");
    const verifyPassword = data.get("verifyPassword");

    if (password !== verifyPassword) {
      return fail(400, { error: "Passwords do not match" });
    }

    try {
      const response = await api.post("/auth/adduser", {
        username,
        password,
        token: "",
      });
      const token = response.data.token;

      // Set the token as a cookie
      cookies.set("token", token, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        secure: process.env.NODE_ENV === "production",
        maxAge: 60 * 60 * 24 * 7, // 1 week
      });

      return redirect(303, "/allegro");
    } catch {
      return fail(400, { error: "Failed to create admin user" });
    }
  },
};
