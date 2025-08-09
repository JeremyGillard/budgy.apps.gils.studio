defmodule BudgyWeb.TransactionLive.Show do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Transaction {@transaction.id}
        <:subtitle>This is a transaction record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/transactions"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/transactions/#{@transaction}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit transaction
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Number">{@transaction.number}</:item>
        <:item title="Statement number">{@transaction.statement_number}</:item>
        <:item title="Value date">{@transaction.value_date}</:item>
        <:item title="Posting date">{@transaction.posting_date}</:item>
        <:item title="Amount">{@transaction.amount}</:item>
        <:item title="Currency">{@transaction.currency}</:item>
        <:item title="Description">{@transaction.description}</:item>
        <:item title="Communication">{@transaction.communication}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Show Transaction")
     |> assign(:transaction, Banking.get_transaction!(id))}
  end
end
