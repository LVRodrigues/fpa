import { PassedInitialConfig } from 'angular-auth-oidc-client';

export const authConfig: PassedInitialConfig = {
	config: {
		authority: 'http://localhost:8080/realms/default', 
		redirectUrl: window.location.origin,
		postLogoutRedirectUri: window.location.origin,
		clientId: 'fpa-client',
		scope: 'openid profile offline_access ', // + your scopes
		responseType: 'code',
		silentRenew: true,
		useRefreshToken: true,
		renewTimeBeforeTokenExpiresInSeconds: 30,
		secureRoutes: ['http://localhost:8080', 'http://localhost:5000'],
	}
};