<script lang="ts">
    import { Alert } from "$lib/ui/ui/alert/index.js";
    import { Button } from "$lib/ui/ui/button/index.js";
    import * as Card from "$lib/ui/ui/card/index.js";
    import { Input } from "$lib/ui/ui/input/index.js";
    import { Label } from "$lib/ui/ui/label/index.js";

    import axios from "axios";
    import { onMount } from "svelte";

    let username: string = "";
    let password: string = "";
    let verifyPassword: string = "";

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

    let noMatch: boolean = false;

    // handle admin creation event
    function createAdmin() {
        // ensure passwords match
        if (password !== verifyPassword) {
            noMatch = true;
            return;
        }

        // create the user
        const data = {
            username: username,
            password: password,
            token: "",
        };
        try {
            api.post("/auth/adduser", data).then((res) => {
                console.log(res.data);
            });
        } catch (error) {
            console.error(error);
        }
    }

    // handle login event
    function handleLogin() {
        const data = {
            username: username,
            password: password,
        };
        try {
            api.post("/auth/login", data).then((res) => {
                console.log(res.data);
            });
        } catch (error) {
            console.error(error);
        }
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-black p-4">
    <Card.Root class="w-full max-w-sm bg-black text-white border-white/20">
        {#if isFirstUser}
            {#if noMatch}
                <Alert.Root>
                    <div class="flex justify-between">
                        <div>
                            <Alert.Title>Error</Alert.Title>
                            <Alert.Description>
                                Passwords do not match.
                            </Alert.Description>
                        </div>
                        <button
                            on:click={() => (noMatch = false)}
                            class="text-gray-500 hover:text-gray-700"
                        >
                            ✕
                        </button>
                    </div>
                </Alert.Root>
            {/if}
            <Card.Header>
                <Card.Title class="text-2xl">Admin User Creation</Card.Title>
                <Card.Description class="text-gray-400"
                    >Please create the administrator account.</Card.Description
                >
            </Card.Header>
            <Card.Content class="grid gap-4">
                <div class="grid gap-2">
                    <Label for="username">Username</Label>
                    <Input
                        id="username"
                        type="username"
                        placeholder="me@example.com"
                        required
                        class="bg-black border-white/20"
                        bind:value={username}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="password">Password</Label>
                    <Input
                        id="password"
                        type="password"
                        required
                        class="bg-black border-white/20"
                        bind:value={password}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="password">Verify Password</Label>
                    <Input
                        id="password"
                        type="password"
                        required
                        class="bg-black border-white/20"
                        bind:value={verifyPassword}
                    />
                </div>
            </Card.Content>
            <Card.Footer>
                <Button class="w-full" type="submit" on:click={createAdmin}
                    >Create</Button
                >
            </Card.Footer>
        {:else}
            <Card.Header>
                <Card.Title class="text-2xl">Login</Card.Title>
                <Card.Description class="text-gray-400"
                    >Enter your username below to login to your account.</Card.Description
                >
            </Card.Header>
            <Card.Content class="grid gap-4">
                <div class="grid gap-2">
                    <Label for="username">Username</Label>
                    <Input
                        id="username"
                        type="username"
                        placeholder="me@example.com"
                        required
                        class="bg-black border-white/20"
                        bind:value={username}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="password">Password</Label>
                    <Input
                        id="password"
                        type="password"
                        required
                        class="bg-black border-white/20"
                        bind:value={password}
                    />
                </div>
            </Card.Content>
            <Card.Footer>
                <Button class="w-full" type="submit" on:click={handleLogin}
                    >Sign in</Button
                >
            </Card.Footer>
        {/if}
    </Card.Root>
</div>
