<script lang="ts">
    import { Button } from "$lib/ui/ui/button/index.js";
    import * as Card from "$lib/ui/ui/card/index.js";
    import { Input } from "$lib/ui/ui/input/index.js";
    import { Label } from "$lib/ui/ui/label/index.js";

    let username: string = "";
    let password: string = "";

    function handleLogin() {
        console.log(username);
        console.log(password);
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-black p-4">
    <Card.Root class="w-full max-w-sm bg-black text-white border-white/20">
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
    </Card.Root>
</div>
