defmodule BudgyWeb.CounterpartyLive.Form do
  use BudgyWeb, :live_view

  alias Budgy.Banking
  alias Budgy.Banking.Counterparty

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        {@page_title}
        <:subtitle>Use this form to manage counterparty records in your database.</:subtitle>
      </.header>

      <.form for={@form} id="counterparty-form" phx-change="validate" phx-submit="save">
        <.input field={@form[:name]} type="text" label="Name" />
        <.input field={@form[:account]} type="text" label="Account" />
        <footer>
          <.button phx-disable-with="Saving..." variant="primary">Save Counterparty</.button>
          <.button navigate={return_path(@return_to, @counterparty)}>Cancel</.button>
        </footer>
      </.form>
    </Layouts.app>
    """
  end

  @impl true
  def mount(params, _session, socket) do
    {:ok,
     socket
     |> assign(:return_to, return_to(params["return_to"]))
     |> apply_action(socket.assigns.live_action, params)}
  end

  defp return_to("show"), do: "show"
  defp return_to(_), do: "index"

  defp apply_action(socket, :edit, %{"id" => id}) do
    counterparty = Banking.get_counterparty!(id)

    socket
    |> assign(:page_title, "Edit Counterparty")
    |> assign(:counterparty, counterparty)
    |> assign(:form, to_form(Banking.change_counterparty(counterparty)))
  end

  defp apply_action(socket, :new, _params) do
    counterparty = %Counterparty{}

    socket
    |> assign(:page_title, "New Counterparty")
    |> assign(:counterparty, counterparty)
    |> assign(:form, to_form(Banking.change_counterparty(counterparty)))
  end

  @impl true
  def handle_event("validate", %{"counterparty" => counterparty_params}, socket) do
    changeset = Banking.change_counterparty(socket.assigns.counterparty, counterparty_params)
    {:noreply, assign(socket, form: to_form(changeset, action: :validate))}
  end

  def handle_event("save", %{"counterparty" => counterparty_params}, socket) do
    save_counterparty(socket, socket.assigns.live_action, counterparty_params)
  end

  defp save_counterparty(socket, :edit, counterparty_params) do
    case Banking.update_counterparty(socket.assigns.counterparty, counterparty_params) do
      {:ok, counterparty} ->
        {:noreply,
         socket
         |> put_flash(:info, "Counterparty updated successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, counterparty))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp save_counterparty(socket, :new, counterparty_params) do
    case Banking.create_counterparty(counterparty_params) do
      {:ok, counterparty} ->
        {:noreply,
         socket
         |> put_flash(:info, "Counterparty created successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, counterparty))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp return_path("index", _counterparty), do: ~p"/counterparties"
  defp return_path("show", counterparty), do: ~p"/counterparties/#{counterparty}"
end
