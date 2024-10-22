<script lang="ts">
    import { Button } from "$lib/ui/ui/button/index.js";
    import * as Card from "$lib/ui/ui/card/index.js";
    import { Input } from "$lib/ui/ui/input/index.js";
    import { Label } from "$lib/ui/ui/label/index.js";

    import axios from "axios";
    import { enhance } from "$app/forms";
    import { type ActionResult } from "@sveltejs/kit";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    // if the first user, show admin onboarding page
    let isFirstUser: boolean = false;

    onMount(async () => {
        try {
            const users: number = await checkFirstUser();
            console.log(users);
            if (users === 0) {
                isFirstUser = true;
            }
        } catch (error) {
            console.error("Error fetching user count:", error);
        }
    });

    // configure axios base URL
    const api = axios.create({
        baseURL: "http://localhost:9000",
    });

    // check if this is the first new user
    async function checkFirstUser() {
        try {
            const res = await api.get("/auth/countuser");
            return res.data.message;
        } catch (error) {
            console.error(error);
            throw error;
        }
    }

    // handle form submission and return value from server
    function handleEnhance() {
        return async ({ result }: { result: ActionResult }) => {
            if (result.type !== "failure") {
                await goto("/allegro");
            }
        };
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-black p-4">
    <Card.Root class="w-full max-w-sm bg-black text-white border-white/20">
        {#if isFirstUser}
            <Card.Header>
                <Card.Title class="text-2xl">Admin User Creation</Card.Title>
                <Card.Description class="text-gray-400"
                    >Please create the administrator account.</Card.Description
                >
            </Card.Header>
            <form
                method="POST"
                action="?/createAdmin"
                use:enhance={handleEnhance}
            >
                <Card.Content class="grid gap-4">
                    <div class="grid gap-2">
                        <Label for="username">Username</Label>
                        <Input
                            id="username"
                            name="username"
                            type="username"
                            placeholder="me@example.com"
                            required
                            class="bg-black border-white/20"
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="password">Password</Label>
                        <Input
                            id="password"
                            name="password"
                            type="password"
                            required
                            class="bg-black border-white/20"
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="verifyPassword">Verify Password</Label>
                        <Input
                            id="verifyPassword"
                            name="verifyPassword"
                            type="password"
                            required
                            class="bg-black border-white/20"
                        />
                    </div>
                </Card.Content>
                <Card.Footer>
                    <Button class="w-full" type="submit">Create</Button>
                </Card.Footer>
            </form>
        {:else}
            <Card.Header>
                <Card.Title class="text-2xl">Login</Card.Title>
                <Card.Description class="text-gray-400"
                    >Enter your username below to login to your account.</Card.Description
                >
            </Card.Header>
            <form method="POST" action="?/login" use:enhance={handleEnhance}>
                <Card.Content class="grid gap-4">
                    <div class="grid gap-2">
                        <Label for="username">Username</Label>
                        <Input
                            id="username"
                            name="username"
                            type="username"
                            placeholder="me@example.com"
                            required
                            class="bg-black border-white/20"
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="password">Password</Label>
                        <Input
                            id="password"
                            name="password"
                            type="password"
                            required
                            class="bg-black border-white/20"
                        />
                    </div>
                </Card.Content>
                <Card.Footer>
                    <Button class="w-full" type="submit">Sign in</Button>
                </Card.Footer>
            </form>
        {/if}
    </Card.Root>
</div>
