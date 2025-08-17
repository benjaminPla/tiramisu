declare global {
	namespace App {
	}
}
declare module '$env/dynamic/public' {
	export const PUBLIC_API_URL: string;
}

export {};
