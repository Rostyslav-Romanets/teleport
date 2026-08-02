// Teleport
// Copyright (C) 2026 Gravitational, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

package web

import (
	"net/http"

	"github.com/gravitational/trace"
	"github.com/gravitational/teleport/api/types"
	"github.com/julienschmidt/httprouter"
)

// windowsCACRL returns an empty CRL for the Windows CA cert.
func (h *Handler) windowsCACRL(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	err := rateLimitRequest(r, h.limiter)
	if err != nil {
		return trace.Wrap(err)
	}

	ctx := r.Context()

	crl, err := h.cfg.ProxyClient.GenerateCertAuthorityCRL(ctx, types.WindowsCA)
	if err != nil {
		h.logger.ErrorContext(ctx, "Failed to generate Windows CA CRL", "error", err)
		http.Error(w, err.Error(), trace.ErrorToCode(err))
		return
	}

	w.Header().Set("Content-Type", "application/pkix-crl")
	if _, err := w.Write(crl); err != nil {
		h.logger.ErrorContext(ctx, "Failed to send Windows CA CRL", "error", err)
	}
}
