defmodule BudgyWeb.TransactionLive.Index do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Listing Transactions
        <:actions>
          <.button variant="primary" navigate={~p"/transactions/new"}>
            <.icon name="hero-plus" /> New Transaction
          </.button>
        </:actions>
      </.header>

      <.table
        id="transactions"
        rows={@streams.transactions}
        row_click={fn {_id, transaction} -> JS.navigate(~p"/transactions/#{transaction}") end}
      >
        <:col :let={{_id, transaction}} label="Number">{transaction.number}</:col>
        <:col :let={{_id, transaction}} label="Statement number">{transaction.statement_number}</:col>
        <:col :let={{_id, transaction}} label="Value date">{transaction.value_date}</:col>
        <:col :let={{_id, transaction}} label="Posting date">{transaction.posting_date}</:col>
        <:col :let={{_id, transaction}} label="Amount">{transaction.amount}</:col>
        <:col :let={{_id, transaction}} label="Currency">{transaction.currency}</:col>
        <:col :let={{_id, transaction}} label="Description">{transaction.description}</:col>
        <:col :let={{_id, transaction}} label="Communication">{transaction.communication}</:col>
        <:action :let={{_id, transaction}}>
          <div class="sr-only">
            <.link navigate={~p"/transactions/#{transaction}"}>Show</.link>
          </div>
          <.link navigate={~p"/transactions/#{transaction}/edit"}>Edit</.link>
        </:action>
        <:action :let={{id, transaction}}>
          <.link
            phx-click={JS.push("delete", value: %{id: transaction.id}) |> hide("##{id}")}
            data-confirm="Are you sure?"
          >
            Delete
          </.link>
        </:action>
      </.table>
    </Layouts.app>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Listing Transactions")
     |> stream(:transactions, Banking.list_transactions())}
  end

  @impl true
  def handle_event("delete", %{"id" => id}, socket) do
    transaction = Banking.get_transaction!(id)
    {:ok, _} = Banking.delete_transaction(transaction)

    {:noreply, stream_delete(socket, :transactions, transaction)}
  end
end
