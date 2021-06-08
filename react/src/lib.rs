use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T>(T);

struct ComputeCell<'a, T> {
    dependencies: Vec<CellID>,
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks: Vec<Option<Box<dyn 'a + FnMut(T)>>>,
    cached_value: Option<T>,
}

impl<'a, T> ComputeCell<'a, T> {
    fn add_callback(&mut self, boxed_fn: Box<dyn 'a + FnMut(T)>) -> CallbackID {
        for (i, cb) in self.callbacks.iter_mut().enumerate() {
            if cb.is_none() {
                *cb = Some(boxed_fn);
                return CallbackID(i);
            }
        }
        self.callbacks.push(Some(boxed_fn));
        CallbackID(self.callbacks.len() - 1)
    }
}

pub struct Reactor<'a, T> {
    inputs: Vec<InputCell<T>>,
    computes: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            inputs: Vec::new(),
            computes: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.inputs.push(InputCell(initial));
        InputCellID(self.inputs.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        dependencies.iter().try_for_each(|cell_id| match cell_id {
            CellID::Input(InputCellID(index)) => self
                .inputs
                .get(*index)
                .map_or_else(|| Err(*cell_id), |_| Ok(())),
            CellID::Compute(ComputeCellID(index)) => self
                .computes
                .get(*index)
                .map_or_else(|| Err(*cell_id), |_| Ok(())),
        })?;
        self.computes.push(ComputeCell {
            dependencies: dependencies.into(),
            compute_func: Box::new(compute_func),
            callbacks: Vec::new(),
            cached_value: None,
        });
        let id = ComputeCellID(self.computes.len() - 1);
        self.computes[id.0].cached_value = self.value(CellID::Compute(id));
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(index)) => Some(self.inputs.get(index)?.0),
            CellID::Compute(ComputeCellID(index)) => {
                let cell = self.computes.get(index)?;
                let values = cell.dependencies.iter().try_fold(vec![], |mut acc, id| {
                    let v = self.value(*id)?;
                    acc.push(v);
                    Some(acc)
                })?;
                Some((cell.compute_func)(values.as_slice()))
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if self
            .inputs
            .get_mut(id.0)
            .map(|cell| cell.0 = new_value)
            .is_some()
        {
            self.propagate_changes(id);
            true
        } else {
            false
        }
    }

    fn propagate_changes(&mut self, id: InputCellID) -> Option<()> {
        self.calculate_change_propagation_order(id)
            .iter()
            .try_for_each(|i| {
                let value = self.value(CellID::Compute(*i))?;
                let cached_value = self.computes[i.0].cached_value?;
                if value != cached_value {
                    self.computes[i.0].callbacks.iter_mut().for_each(|cb| {
                        if let Some(callback) = cb {
                            (callback)(value);
                        }
                    });
                    self.computes[i.0].cached_value = Some(value);
                }
                Some(())
            })?;
        Some(())
    }

    fn calculate_change_propagation_order(&mut self, id: InputCellID) -> Vec<ComputeCellID> {
        let input_id = CellID::Input(id);
        let mut deps = self
            .computes
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.dependencies.contains(&input_id))
            .map(|(i, _)| ComputeCellID(i))
            .collect::<Vec<_>>();

        let mut graph = HashMap::<ComputeCellID, Vec<ComputeCellID>>::new();
        let mut visited = HashMap::<ComputeCellID, Cell<bool>>::new();
        let stack = RefCell::new(Vec::<ComputeCellID>::new());

        //generate deps graph
        while let Some(d_id) = deps.pop() {
            let mut dependents = self
                .computes
                .iter()
                .enumerate()
                .filter(|(_, cell)| cell.dependencies.contains(&CellID::Compute(d_id)))
                .map(|(i, _)| ComputeCellID(i))
                .collect::<Vec<_>>();
            deps.append(&mut dependents);

            let own_computed_deps =
                self.computes[d_id.0]
                    .dependencies
                    .iter()
                    .fold(vec![], |mut acc, id| {
                        if let CellID::Compute(id) = id {
                            acc.push(*id);
                            acc
                        } else {
                            acc
                        }
                    });
            if !graph.contains_key(&d_id) {
                graph.insert(d_id, own_computed_deps);
                visited.insert(d_id, Cell::new(false));
            }
        }

        //remove unneeded deps
        let only_needed_nodes = graph.keys().copied().collect::<Vec<_>>();
        graph.values_mut().for_each(|v| {
            let deps = v
                .iter()
                .filter(|x| only_needed_nodes.contains(x))
                .copied()
                .collect::<Vec<_>>();
            *v = deps;
        });

        //topological sorting
        fn topo_sort_step(
            v: ComputeCellID,
            visited: &HashMap<ComputeCellID, Cell<bool>>,
            stack: &RefCell<Vec<ComputeCellID>>,
            graph: &HashMap<ComputeCellID, Vec<ComputeCellID>>,
        ) {
            visited[&v].set(true);

            for i in &graph[&v] {
                if !visited[i].get() {
                    topo_sort_step(*i, visited, stack, graph);
                }
            }
            stack.borrow_mut().push(v);
        }

        for i in visited.keys() {
            if !visited[i].get() {
                topo_sort_step(*i, &visited, &stack, &graph);
            }
        }

        stack.into_inner()
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.computes
            .get_mut(id.0)
            .map(|cell| cell.add_callback(Box::new(callback)))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let _ = self
            .computes
            .get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)?
            .callbacks
            .get_mut(callback.0)
            .ok_or(RemoveCallbackError::NonexistentCallback)?
            .take()
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        Ok(())
    }
}
