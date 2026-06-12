import { sveltekit } from '@sveltejs/kit/vite';
import netlify from '@netlify/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), netlify()]
});
