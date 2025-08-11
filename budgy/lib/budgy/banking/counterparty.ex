defmodule Budgy.Banking.Counterparty do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "counterparties" do
    field :name, :string
    field :address_id, :binary_id

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(counterparty, attrs) do
    counterparty
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
