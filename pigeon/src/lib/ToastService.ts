import { type ToastSettings } from '@skeletonlabs/skeleton';

export function get_request_sent_notification(): ToastSettings {
	return {
		message: '📤 Sent request',
		timeout: 3000,
		background: 'variant-filled-success'
	};
}

export function get_failure_to_send_request_notification(message: string): ToastSettings {
	return {
		message: '😭 ' + message,
		timeout: 3000,
		background: 'variant-filled-primary'
	};
}

export function get_failure_formatting_json_notification(): ToastSettings {
	return {
		message: '😭 Failed to format json',
		timeout: 3000,
		background: 'variant-filled-primary'
	};
}