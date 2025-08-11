defmodule Budgy.Banking.Transaction do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "transactions" do
    field :number, :integer
    field :statement_number, :integer
    field :value_date, :utc_datetime
    field :posting_date, :utc_datetime
    field :amount, :decimal
    field :currency, :string
    field :description, :string
    field :communication, :string
    belongs_to :sender, Budgy.Banking.Counterparty
    belongs_to :recipient, Budgy.Banking.Counterparty

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(transaction, attrs) do
    transaction
    |> cast(attrs, [:number, :statement_number, :value_date, :posting_date, :amount, :currency, :description, :communication, :sender_id, :recipient_id])
    |> validate_required([:number, :statement_number, :value_date, :posting_date, :amount, :currency, :description, :sender_id, :recipient_id])
  end
end
