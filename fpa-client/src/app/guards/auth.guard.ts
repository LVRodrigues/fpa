import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { OidcSecurityService } from 'angular-auth-oidc-client';
import { map, take } from 'rxjs';

export const isAuthenticated = () => {
    const security = inject(OidcSecurityService);
    const router = inject(Router);

    return security.isAuthenticated$.pipe(
        take(1),
        map(({ isAuthenticated }) => {
            if (isAuthenticated) {
                return true;
            }
            return router.parseUrl('/forbidden');
        })
    );
};