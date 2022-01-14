package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type (
	EnvironmentIn  openapi.EnvironmentIn
	EnvironmentOut openapi.EnvironmentOut
	SettingsIn     openapi.SettingsIn
	SettingsOut    openapi.SettingsOut
)

type Environment struct {
	api *openapi.APIClient
}

func (e *Environment) Import(environmentIn *EnvironmentIn) error {
	req := e.api.EnvironmentApi.ImportEnvironmentConfigurationApiV1EnvironmentImportPost(context.Background())
	req = req.EnvironmentIn(openapi.EnvironmentIn(*environmentIn))

	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Environment) Export() (*EnvironmentOut, error) {
	req := e.api.EnvironmentApi.ExportEnvironmentConfigurationApiV1EnvironmentExportPost(context.Background())
	req = req.Body(map[string]interface{}{})
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EnvironmentOut(resp)
	return &ret, nil
}
