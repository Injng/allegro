import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:9000",
});

export const actions: Actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get("username");
    const password = data.get("password");

    // ensure username is not empty
    if (username === "" || username === null) {
      return fail(400, { error: "Username cannot be empty" });
    }

    // ensure password is not empty
    if (password === "" || password === null) {
      return fail(400, { error: "Password cannot be empty" });
    }

    try {
      const response = await api.post("/auth/login", { username, password });
      const token = response.data.token;

      console.log(response);
      // ensure the response is successful
      if (!response.data.access) {
        return fail(400, { error: "Login failed" });
      }

      // Set the token as a cookie
      cookies.set("token", token, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        secure: process.env.NODE_ENV === "production",
        maxAge: 60 * 60 * 24 * 7, // 1 week
      });

      return { success: true };
    } catch {
      return fail(400, { error: "Server error" });
    }
  },

  createAdmin: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get("username");
    const password = data.get("password");
    const verifyPassword = data.get("verifyPassword");

    if (username === "") {
      return fail(400, { error: "Username cannot be empty" });
    }

    if (password === "") {
      return fail(400, { error: "Password cannot be empty" });
    }

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

      // ensure the response is successful
      if (!response.data.success) {
        return fail(400, { error: "Add user failed" });
      }

      // Set the token as a cookie
      cookies.set("token", token, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        secure: process.env.NODE_ENV === "production",
        maxAge: 60 * 60 * 24 * 7, // 1 week
      });

      return { success: true };
    } catch {
      return fail(400, { error: "Failed to create admin user" });
    }
  },
};
