


import {
  createSignal,
  onCleanup,
  createMemo,
} from "solid-js";
import h from "solid-js/h";

function ToDoList(props){

  const [content, setContent] = createSignal();
  const [tasks, setTasks] = createSignal([
    {
      id: crypto.randomUUID(),
      content:"test1"
    },
    {
      id: crypto.randomUUID(),
      content:"test2"
    }
  ]);


  const RenderTasks = createMemo(() => {
    console.log("render task...")
    let tmp = tasks();
    let list = [];
    for (let i=0; i < tmp.length;i++ ){
      //console.log(tmp[i]);
      list.push(
        h("li",{},
          h("label",{id:tmp[i].id}, tmp[i].content, h("button",{onclick:()=>delete_task(tmp[i].id)},"Delete"))
        )
      )
    }
    console.log(list)
    return list;
  });

  function addTask(){
    console.log("add...");
    //let tmp = tasks();

    //tmp.push({
      //id: crypto.randomUUID(),
      //content:"test"
    //})

    //setTasks(tmp)
    setTasks(state=>[...state,
      {
        id: crypto.randomUUID(),
        content:"test " + crypto.randomUUID()
      }
    ])

    //console.log(tmp)
  }

  function delete_task(id){
    console.log(id)
    setTasks(state=>state.filter(item=>item.id != id));
  }

  return h("div",{}, 
    h("div",{}, 
      h("label",{}, 
        "Content"
      ),
      h("input",{}),
      h("button",{onclick:addTask},"Add"),
    ),
    h("div",{}, 
      h("ul",{},
        RenderTasks
      )
    ),
  )
}

export default ToDoList;