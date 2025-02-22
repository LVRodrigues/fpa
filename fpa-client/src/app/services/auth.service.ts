import { Injectable, OnInit } from '@angular/core';
import { OidcSecurityService } from 'angular-auth-oidc-client';

@Injectable({
	providedIn: 'root'
})
export class AuthService {

	constructor(
		private readonly security: OidcSecurityService
	) { }

	login() {
		this.security.authorize();
	}

	logout() {
		this.security.logoff()
			.subscribe((result) => console.log(result));
	}

	isAuthenticated(): boolean {
		let authenticated = this.security.authenticated;
		return authenticated().isAuthenticated;
	}

	username(): string | undefined {
		let data = this.security.userData;
		return data().userData?.name;
	}
}
