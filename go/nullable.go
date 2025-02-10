package svix

import "github.com/svix/svix-webhooks/go/models"

func NewNullable[T any](value T) models.Nullable[T] {
	return models.NewNullable(value)
}
